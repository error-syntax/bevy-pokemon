use bevy::prelude::*;

#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Eq, Hash, PartialEq)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct WalkAnimation {
    pub(crate) direction: MoveDirection,
    pub(crate) current: usize,
}
