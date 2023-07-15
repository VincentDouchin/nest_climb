use crate::*;
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        texture::DEFAULT_IMAGE_HANDLE,
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

#[derive(Component)]
pub struct LinkedEntity(Entity);

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct NineSliceMaterial {
    #[uniform(0)]
    pub margins: Vec4,
    #[uniform(0)]
    pub size: Vec2,
    #[uniform(0)]
    pub scale: Vec2,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Handle<Image>,
}

impl Material2d for NineSliceMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/nine_slice.wgsl".into()
    }
}

#[derive(Component, Clone)]
pub struct NineSlice {
    pub image_handle: Handle<Image>,
    pub margins: Vec4,
    pub layer: u8,
    pub scale: f32,
}
impl Default for NineSlice {
    fn default() -> Self {
        NineSlice {
            image_handle: DEFAULT_IMAGE_HANDLE.typed(),
            margins: Vec4::splat(0.0),
            layer: 1,
            scale: 1.0,
        }
    }
}
#[derive(Component)]
pub struct NineSliceLoaded;

pub fn create_render_texture(
    size: Vec2,
    maybe_translation: Option<Vec3>,
    layer: u8,
    mut images: ResMut<Assets<Image>>,
    commands: &mut Commands,
) -> (Handle<Image>, RenderLayers, Entity) {
    let translation = maybe_translation.unwrap_or(Vec3::splat(0.0));
    let size = Extent3d {
        width: size.x as u32,
        height: size.y as u32,
        ..default()
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);
    let first_pass_layer = RenderLayers::layer(layer);
    let image_handle = images.add(image);
    let camera = commands
        .spawn((
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::NONE),
                    ..default()
                },

                camera: Camera {
                    // render before the "main pass" camera
                    order: -1,
                    target: RenderTarget::Image(image_handle.clone()),
                    ..default()
                },
                transform: Transform::from_translation(translation + Vec3::Z)
                    .looking_at(translation, Vec3::Y),
                ..default()
            },
            UiCameraConfig { show_ui: false },
            first_pass_layer,
        ))
        .id();

    return (image_handle, first_pass_layer, camera);
}

pub fn create_nine_slice(
    mut commands: Commands,
    mut query: Query<(Entity, &Node, &Transform, &NineSlice), Without<NineSliceLoaded>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NineSliceMaterial>>,
) {
    for (entity, node, original_transform, nine_slice) in query.iter_mut() {
        let translation = original_transform.translation + Vec3::new(10000.0, 10000.0, 0.0);
        let nine_slice_size = node.size();
        if nine_slice_size.x == 0.0 || nine_slice_size.y == 0.0 {
            continue;
        }
        let size_vec = images
            .get(&nine_slice.image_handle)
            .expect("get image from handle")
            .size();
        let size = Extent3d {
            width: nine_slice_size.x as u32,
            height: nine_slice_size.y as u32,
            ..default()
        };
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..default()
        };
        image.resize(size);

        let image_handle = images.add(image);
        let first_pass_layer = RenderLayers::layer(nine_slice.layer);
        let material_handle = materials.add(NineSliceMaterial {
            margins: nine_slice.margins,
            size: size_vec,
            scale: Vec2::new(
                nine_slice_size.x / size_vec.x / nine_slice.scale,
                nine_slice_size.y / size_vec.y / nine_slice.scale,
            ),
            color_texture: nine_slice.image_handle.clone(),
        });
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform::from_translation(translation).with_scale(Vec3::new(
                    nine_slice_size.x,
                    nine_slice_size.y,
                    0.0,
                )),

                material: material_handle,
                ..Default::default()
            },
            first_pass_layer,
            LinkedEntity(entity),
        ));
        commands.spawn((
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::NONE),
                    ..default()
                },

                camera: Camera {
                    // render before the "main pass" camera
                    order: -1,
                    target: RenderTarget::Image(image_handle.clone()),
                    ..default()
                },
                transform: Transform::from_translation(translation + Vec3::Z)
                    .looking_at(translation, Vec3::Y),
                ..default()
            },
            UiCameraConfig { show_ui: false },
            first_pass_layer,
            LinkedEntity(entity),
        ));

        commands
            .entity(entity)
            .insert(NineSliceLoaded)
            .insert(BackgroundColor(Color::NONE))
            .insert(UiImage::new(image_handle));
    }
}

pub fn display_nine_slice(query: Query<Entity, Added<NineSliceLoaded>>, mut commands: Commands) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(BackgroundColor(Color::WHITE));
    }
}

pub fn despawn_nine_slice(query: Query<(Entity, &LinkedEntity)>, mut commands: Commands) {
    for (entity, linked_entity) in query.iter() {
        if commands.get_entity(linked_entity.0).is_none() {
            commands.entity(entity).despawn_recursive()
        }
    }
}

pub fn update_nine_slice(
    changed_query: Query<(Entity, &NineSlice), Changed<NineSlice>>,
    mut nine_slice_query: Query<(&mut Handle<NineSliceMaterial>, &LinkedEntity)>,
    mut materials: ResMut<Assets<NineSliceMaterial>>,
) {
    for (changed_entity, changed_nine_slice) in changed_query.iter() {
        for (nine_slice_material, linked_entity) in nine_slice_query.iter_mut() {
            if changed_entity == linked_entity.0 {
                if let Some(mut material) = materials.get_mut(&nine_slice_material) {
                    material.color_texture = changed_nine_slice.image_handle.clone();
                }
            }
        }
    }
}

pub fn nine_slice_plugin(app: &mut App) {
    app.add_plugin(Material2dPlugin::<NineSliceMaterial>::default())
        .add_system(create_nine_slice)
        .add_system(display_nine_slice)
        .add_system(update_nine_slice)
        .add_system(despawn_nine_slice);
}
