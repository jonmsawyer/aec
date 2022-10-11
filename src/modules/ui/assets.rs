use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct SpriteCollection {
    /// Light and dark squares, with chess pieces are defined in a texture atlas (aka sprite sheet).
    ///
    /// Consts cannot be used in attribute macros, so we have to hardcode tile size into here
    #[asset(texture_atlas(tile_size_x = 256., tile_size_y = 256., columns = 14, rows = 1))]
    #[asset(path = "default_board.png")]
    pub tiles: Handle<TextureAtlas>,
    // /// Atlas for our character sprites
    // #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 6, rows = 10))]
    // #[asset(path = "characters.png")]
    // pub characters: Handle<TextureAtlas>,
    // /// Texture atlas for our cursor/selection sprites
    // #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 5, rows = 1))]
    // #[asset(path = "cursor.png")]
    // pub cursors: Handle<TextureAtlas>,
    // /// The background image
    // #[asset(path = "nasa-mars.png")]
    // pub background: Handle<Image>,
}
