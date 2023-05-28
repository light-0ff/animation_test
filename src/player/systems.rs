use bevy::prelude::*;

use super::components::*;
use super::resources::*;
use crate::animation::*;

const MOVE_SPEED: f32 = 100.;
const FALL_SPEED: f32 = 98.0;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    animaitons: Res<PlayerAnimations>,
) {
    let Some(animation) = animaitons.get(Animation::Idle) else {error!("Failed to find animation: Idle"); return;};
    let texture_handle = asset_server.load("sprites/player/adventurer-Sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 37.0), 7, 16, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        },
        Player {},
        animation,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

pub fn move_player(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Transform), With<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    let (entity, mut player) = player.single_mut();
    if input.any_just_pressed([KeyCode::W, KeyCode::Up, KeyCode::Space]) {
        commands.entity(entity).insert(Jump(100.));
    } else if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        player.translation.x -= MOVE_SPEED * time.delta_seconds();
    } else if input.any_pressed([KeyCode::D, KeyCode::Right]) {
        player.translation.x += MOVE_SPEED * time.delta_seconds();
    }
}

pub fn change_player_animation(
    mut player: Query<
        (
            &mut Handle<TextureAtlas>,
            &mut AnimationIndices,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    player_jump: Query<(&Transform, Option<&Jump>), With<Player>>,
    input: Res<Input<KeyCode>>,
    animaitons: Res<PlayerAnimations>,
) {
    let (mut atlas, mut animation, mut sprite) = player.single_mut();
    let (pos, jump) = player_jump.single();

    if input.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        sprite.flip_x = true;
    } else if input.any_just_pressed([KeyCode::D, KeyCode::Right])
        && !input.any_pressed([KeyCode::A, KeyCode::Left])
    {
        sprite.flip_x = false;
    } else if input.any_just_released([KeyCode::A, KeyCode::Left])
        && !input.any_pressed([KeyCode::A, KeyCode::Left])
        && input.any_pressed([KeyCode::D, KeyCode::Right])
    {
        sprite.flip_x = false;
    }

    //Jumping if jump
    if jump.is_some() {
        let Some(new_animaiton) = animaitons.get(Animation::Jump) else {error!("No Animation Jump Loaded"); return;};
        *animation = new_animaiton;
        sprite.index = new_animaiton.first;
        return;
    //Falling if Y > 0.0
    } else if pos.translation.y > 0.0 {
        let Some(new_animaiton) = animaitons.get(Animation::Fall) else {error!("No Animation Fall Loaded"); return;};
        *animation = new_animaiton;
        sprite.index = new_animaiton.first;
        return;
    }

    // if any move keys pressed set run sprite
    if input.any_just_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        let Some(new_animaiton) = animaitons.get(Animation::Run) else {error!("No Animation Run Loaded"); return;};
        *animation = new_animaiton;
        sprite.index = new_animaiton.first;
    }

    //if no move keys pressed set idel animtaion
    if input.any_just_released([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right])
        && !input.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right])
    {
        let Some(new_animaiton) = animaitons.get(Animation::Idle) else {error!("No Animation Idle Loaded"); return;};
        *animation = new_animaiton;
        sprite.index = new_animaiton.first;
    }
}

pub fn player_jump(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Transform, &mut Jump), With<Player>>,
    time: Res<Time>,
) {
    let Ok((player, mut transform,mut jump)) = player.get_single_mut() else {return;};
    let jump_power = (time.delta_seconds() * FALL_SPEED * 2.).min(jump.0);
    jump.0 -= jump_power;
    transform.translation.y += jump_power;
    if jump.0 == 0. {
        commands.entity(player).remove::<Jump>();
    }
}

pub fn player_fall(
    mut player: Query<&mut Transform, (With<Player>, Without<Jump>)>,
    time: Res<Time>,
) {
    let Ok(mut player) = player.get_single_mut() else {return;};
    if player.translation.y > 0.0 {
        player.translation.y -= time.delta_seconds() * FALL_SPEED;
        if player.translation.y < 0.0 {
            player.translation.y = 0.0
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Animation {
    Run,
    Idle,
    Jump,
    Fall,
}
