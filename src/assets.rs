use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct Textures {
    #[asset(path = "textures/table.png")]
    pub table: Handle<Image>,
    #[asset(path = "textures/center_circle.png")]
    pub center_circle: Handle<Image>,
    #[asset(path = "textures/goal_post.png")]
    pub goal_post: Handle<Image>,
    #[asset(path = "textures/puck.png")]
    pub puck: Handle<Image>,
    #[asset(path = "textures/stick.png")]
    pub stick: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct Fonts {
    #[asset(path = "fonts/arial.ttf")]
    pub arial: Handle<Font>,
}
