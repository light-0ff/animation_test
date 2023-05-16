mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Configure System Sets
            .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            // Startup Systems
            .add_startup_system(spawn_player)
            // Systems
            .add_system(player_movement.in_set(MovementSystemSet));
    }
}
