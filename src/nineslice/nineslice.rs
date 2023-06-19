use crate::*;
use bevy::{
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct NineSliceMaterial {
    // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
    // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
    #[uniform(0)]
    color: Color,
    // Images can be bound as textures in shaders. If the Image's sampler is also needed, just
    // add the sampler attribute with a different binding index.
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
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
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(NineSliceMaterial {
            color: Color::RED,
            color_texture: assets.button_normal.clone(),
        }),
        ..Default::default()
    });
}

pub fn nine_slice_plugin(app: &mut App) {
    app.add_plugin(Material2dPlugin::<NineSliceMaterial>::default());
    app.add_system(test_nine_slice.in_schedule(OnEnter(GameState::Start)));
}
