use std::collections::HashMap;
use std::sync::OnceLock;

use bevy::prelude::*;

use crate::components::{AnimationTimer, MoveDirection, Player, WalkAnimation};
use crate::constants::ANIM_FPS;

fn walk_frames() -> &'static HashMap<MoveDirection, &'static [usize]> {
    static FRAMES: OnceLock<HashMap<MoveDirection, &'static [usize]>> = OnceLock::new();
    FRAMES.get_or_init(|| {
        HashMap::from([
            (MoveDirection::Down,  &[1usize, 0, 1, 2] as &[usize]),
            (MoveDirection::Left,  &[6usize, 7, 6, 7] as &[usize]),
            (MoveDirection::Right, &[8usize, 9, 8, 9] as &[usize]),
            (MoveDirection::Up,    &[4usize, 3, 4, 5] as &[usize]),
        ])
    })
}

impl WalkAnimation {
    pub fn frames(&self) -> &'static [usize] {
        walk_frames()[&self.direction]
    }

    pub fn set_direction(&mut self, dir: MoveDirection) {
        if self.direction != dir {
            self.direction = dir;
            self.current = 0;
        }
    }

    pub fn advance(&mut self) {
        self.current = (self.current + 1) % self.frames().len();
    }

    pub fn current_frame(&self) -> usize {
        self.frames()[self.current]
    }

    pub fn reset(&mut self) {
        self.current = 0;
    }
}

pub(crate) fn animate_player(
    time: Res<Time>,
    buttons: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut AnimationTimer, &mut WalkAnimation, &mut Sprite), With<Player>>,
) {
    let (mut timer, mut anim, mut sprite) = player.into_inner();
    let moving = buttons.any_pressed([KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD]);

    if moving {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            anim.advance();
        }
    } else {
        anim.reset();
        timer.0.reset();
    }

    if let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index = anim.current_frame();
    }
}

pub fn new_animation_timer() -> AnimationTimer {
    AnimationTimer(Timer::from_seconds(1.0 / ANIM_FPS, TimerMode::Repeating))
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, _app: &mut App) {}
}
