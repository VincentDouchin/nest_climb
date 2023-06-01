use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct CollectibleBundle {
    #[ldtk_entity]
    pub collectible: Collectible,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Collectible {
    pub value: u32,
}

impl LdtkEntity for Collectible {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let value = 10;
        return Collectible { value };
    }
}
