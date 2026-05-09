use std::collections::HashMap;
use std::sync::OnceLock;

use bevy::{
    camera::ScalingMode,
    color::palettes::tailwind::RED_400,
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct AnimationTimer(Timer);

#[derive(Eq, Hash, PartialEq)]
enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct WalkAnimation {
    direction: MoveDirection,
    current: usize,
}

fn walk_frames() -> &'static HashMap<MoveDirection, &'static [usize]> {
    static FRAMES: OnceLock<HashMap<MoveDirection, &'static [usize]>> = OnceLock::new();
    FRAMES.get_or_init(|| {
        HashMap::from([
            (MoveDirection::Down,  &[0usize, 1, 2, 1] as &[usize]),
            (MoveDirection::Left,  &[6usize, 7, 6, 7] as &[usize]),
            (MoveDirection::Right, &[8usize, 9, 8, 9] as &[usize]),
            (MoveDirection::Up,    &[4usize, 3, 4, 5] as &[usize]),
        ])
    })
}

impl WalkAnimation {
    fn frames(&self) -> &'static [usize] {
        walk_frames()[&self.direction]
    }

    fn set_direction(&mut self, dir: MoveDirection) {
        if self.direction != dir {
            self.direction = dir;
            self.current = 0;
        }
    }

    fn advance(&mut self) {
        self.current = (self.current + 1) % self.frames().len();
    }

    fn current_frame(&self) -> usize {
        self.frames()[self.current]
    }

    fn reset(&mut self) {
        self.current = 0;
    }
}

const VELOCITY: f32 = 1.0;
const PLAYER_SIZE: f32 = 15.0;

// ── Sprite sheet config ──────────────────────────────────────────────────────
const SPRITE_W: u32 = 16;
const SPRITE_H: u32 = 16;
const SHEET_COLS: u32 = 10;
const SHEET_ROWS: u32 = 1;
const ANIM_FPS: f32 = 8.0;
// ────────────────────────────────────────────────────────────────────────────

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (handle_player_movement, animate_player, camera_follow, handle_collision).chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_height: 270.,
                max_width: 480.,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(SPRITE_W, SPRITE_H),
        SHEET_COLS,
        SHEET_ROWS,
        None,
        None,
    ));

    let anim = WalkAnimation { direction: MoveDirection::Down, current: 0 };
    let start_index = anim.current_frame();

    commands.spawn((
        Player,
        Sprite {
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            image: asset_server.load("ash_walking_sprite_sheet.png"),
            texture_atlas: Some(TextureAtlas { layout, index: start_index }),
            ..default()
        },
        anim,
        AnimationTimer(Timer::from_seconds(1.0 / ANIM_FPS, TimerMode::Repeating)),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));

    commands.spawn((
        Wall,
        Sprite {
            custom_size: Some(Vec2::splat(30.)),
            color: Color::linear_rgb(255., 255., 255.),
            ..default()
        },
        Transform::from_xyz(0.0, 1.0, 1.0),
    ));
}

fn handle_player_movement(
    mut player: Single<
        (&mut Transform, &mut WalkAnimation),
        (With<Player>, Without<Camera2d>, Without<Wall>),
    >,
    buttons: Res<ButtonInput<KeyCode>>,
) {
    let (mut transform, mut anim) = player.into_inner();
    let mut direction = Vec2::ZERO;

    if buttons.pressed(KeyCode::KeyW) { direction.y += 1.0; }
    if buttons.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
    if buttons.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if buttons.pressed(KeyCode::KeyD) { direction.x += 1.0; }

    if direction != Vec2::ZERO {
        let move_dir = if direction.y > 0.0 {
            MoveDirection::Up
        } else if direction.y < 0.0 {
            MoveDirection::Down
        } else if direction.x < 0.0 {
            MoveDirection::Left
        } else {
            MoveDirection::Right
        };
        anim.set_direction(move_dir);

        direction = direction.normalize();
        transform.translation.x += direction.x * VELOCITY;
        transform.translation.y += direction.y * VELOCITY;
    }
}

fn animate_player(
    time: Res<Time>,
    buttons: Res<ButtonInput<KeyCode>>,
    mut player: Single<(&mut AnimationTimer, &mut WalkAnimation, &mut Sprite), With<Player>>,
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

fn camera_follow(
    player: Single<&Transform, (With<Player>, Without<Camera2d>, Without<Wall>)>,
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>, Without<Wall>)>,
) {
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}

fn handle_collision(
    mut player: Single<
        (&Sprite, &mut Transform),
        (With<Player>, Without<Wall>, Without<Camera2d>),
    >,
    walls: Query<(&Sprite, &Transform), (With<Wall>, Without<Player>, Without<Camera2d>)>,
    mut gizmos: Gizmos,
) {
    let player_pos = player.1.translation.xy();
    let player_radius = PLAYER_SIZE / 2.;
    let player_collider = BoundingCircle::new(player_pos, player_radius);

    gizmos.circle_2d(player_pos, player_radius, RED_400);

    for (sprite, wall_transform) in &walls {
        let wall_pos = wall_transform.translation.xy();
        let wall_half = sprite.custom_size.unwrap() / 2.;
        let wall_collider = Aabb2d::new(wall_pos, wall_half);

        gizmos.rect_2d(wall_pos, sprite.custom_size.unwrap(), RED_400);

        if player_collider.intersects(&wall_collider) {
            let closest = Vec2::new(
                player_pos.x.clamp(wall_pos.x - wall_half.x, wall_pos.x + wall_half.x),
                player_pos.y.clamp(wall_pos.y - wall_half.y, wall_pos.y + wall_half.y),
            );
            let offset = player_pos - closest;
            let dist = offset.length();
            let correction = if dist > 0. {
                offset / dist * (player_radius - dist)
            } else {
                Vec2::new(player_radius, 0.)
            };
            player.1.translation.x += correction.x;
            player.1.translation.y += correction.y;
        }
    }
}
