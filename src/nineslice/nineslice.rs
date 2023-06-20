use crate::*;
use bevy::{
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        render_resource::{AsBindGroup, ShaderRef},
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

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
        "shaders/custom_material.wgsl".into()
    }
}

pub fn test_nine_slice(
    mut commands: Commands,
    mut materials: ResMut<Assets<NineSliceMaterial>>,
    assets: Res<MyAssets>,
    images: Res<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let size = images.get(&assets.button_normal).unwrap().size();
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
    //     transform: Transform::default().with_scale(Vec3::new(64.0, 32.0, 0.0)),
    //     material: materials.add(NineSliceMaterial {
    //         margins: Vec4::splat(8.0),
    //         size,
    //         scale: Vec2::new(2.0, 1.0),
    //         color_texture: assets.button_normal.clone(),
    //     }),
    //     ..Default::default()
    // });
    let material_handle = materials.add(NineSliceMaterial {
        margins: Vec4::splat(8.0),
        size,
        scale: Vec2::new(2.0, 1.0),
        color_texture: assets.button_normal.clone(),
    });
    // let texture = materials
    //     .get(&material_handle)
    //     .unwrap()
    //     .color_texture
    //     .clone();
    commands.spawn(ButtonBundle {
        style: Style {
            size: Size::all(Val::Px(64.0)),
            ..default()
        },
        image: UiImage::new(texture),
        ..default()
    });
}

pub fn nine_slice_plugin(app: &mut App) {
    app.add_plugin(Material2dPlugin::<NineSliceMaterial>::default());
    app.add_system(test_nine_slice.in_schedule(OnEnter(GameState::Start)));
}
