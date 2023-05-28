use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter State
            // Systems
            .add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(change_player_animation)
            .init_resource::<PlayerAnimations>()
            .add_system(player_fall)
            .add_system(player_jump)
            ;
    }
}