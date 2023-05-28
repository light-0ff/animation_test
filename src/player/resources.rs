use bevy::{prelude::*, utils::HashMap};

use crate::animation::*;

use super::systems::*;

#[derive(Resource)]
pub struct PlayerAnimations {
    map: HashMap<Animation, AnimationIndices>,
}
impl PlayerAnimations {
    pub fn add(
        &mut self,
        id: Animation,
        animation: AnimationIndices,
    ) {
        self.map.insert(id, animation);
    }
    pub fn get(&self, id: Animation) -> Option<AnimationIndices> {
        self.map.get(&id).cloned()
    }
}
impl FromWorld for PlayerAnimations {
    fn from_world(world: &mut World) -> Self {
        let mut map = PlayerAnimations {
            map: HashMap::new(),
        };

        map.add(
            Animation::Idle,
            AnimationIndices { first: 0, last: 3 },
        );
        map.add(
            Animation::Run,
            AnimationIndices { first: 8, last: 13 },
        );
        map.add(
            Animation::Jump,
            AnimationIndices {
                first: 14,
                last: 21,
            },
        );
        map.add(
            Animation::Fall,
            AnimationIndices {
                first: 22,
                last: 23,
            },
        );

        map
    }
}
