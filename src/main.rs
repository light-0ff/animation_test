use bevy::prelude::*;

mod animation;
mod player;

use animation::animate_sprite;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(),
        ))
        .add_startup_system(spawn_camera)
        .add_system(animate_sprite)
        .add_plugin(PlayerPlugin)
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
