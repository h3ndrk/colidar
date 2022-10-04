use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct Textures {
    #[asset(path = "textures/table.png")]
    pub table: Handle<Image>,
    #[asset(path = "textures/puck.png")]
    pub puck: Handle<Image>,
    #[asset(path = "textures/stick.png")]
    pub stick: Handle<Image>,
}
