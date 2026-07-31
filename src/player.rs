use bevy::prelude::*;

const TILE_SIZE: u32 = 128;
const WALK_FRAMES: usize = 15;
const SPEED: f32 = 100.0;
const ANIM_DT: f32 = 0.1;
const PLAYER_Z: f32 = 10.0;

pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationState {
    facing: Facing,
    moving: bool,
    was_moving: bool,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, animate_player));
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image> = asset_server.load("knight.png");
    let layout = atlas_layout.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        WALK_FRAMES as u32,
        8,
        None,
        None,
    ));

    let facing = Facing::Down;
    let start_index = atlas_index_of(facing, 0);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: start_index,
            },
        ),
        Transform::from_translation(Vec3::new(0., 0., PLAYER_Z)),
        Player,
        AnimationState {
            facing,
            moving: false,
            was_moving: false,
        },
        AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
    ));
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<(&mut Transform, &mut AnimationState), With<Player>>,
) {
    let (mut transform, mut anim) = player.into_inner();

    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let delta = SPEED * direction.normalize() * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
        anim.facing = facing_from_vector(direction);
        anim.moving = true;
    } else {
        anim.moving = false;
    }
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut AnimationTimer, &mut Sprite), With<Player>>,
) {
    let Ok((mut anim, mut timer, mut sprite)) = query.single_mut() else {
        return;
    };

    let atlas = match sprite.texture_atlas.as_mut() {
        Some(a) => a,
        None => return,
    };

    let target_row = row_zero_based(anim.facing);
    let mut current_row = atlas.index / 15;
    let mut current_col = atlas.index % 15;

    let just_started = anim.moving && !anim.was_moving;
    let just_stopped = !anim.moving && anim.was_moving;

    if current_row != target_row {
        atlas.index = row_start_index(anim.facing);
        current_col = 0;
        current_row = target_row;
        timer.reset()
    }

    if anim.moving {
        if just_started {
            let row_start = row_start_index(anim.facing);
            let next_col = (current_col + 1) % WALK_FRAMES;
            atlas.index = row_start + next_col;
            timer.reset()
        } else {
            timer.tick(time.delta());
            if timer.is_finished() {
                let row_start = row_start_index(anim.facing);
                let next_col = (current_col + 1) % WALK_FRAMES;
                atlas.index = row_start + next_col;
                timer.reset()
            }
        }
    } else if just_stopped {
        timer.reset()
    }
    anim.was_moving = anim.moving;
}

fn row_start_index(facing: Facing) -> usize {
    row_zero_based(facing) * WALK_FRAMES
}

fn atlas_index_of(facing: Facing, frame_in_row: usize) -> usize {
    row_zero_based(facing) + frame_in_row.min(WALK_FRAMES - 1)
}

fn row_zero_based(facing: Facing) -> usize {
    match facing {
        Facing::Right => 0,
        Facing::DownRight => 1,
        Facing::Down => 2,
        Facing::DownLeft => 3,
        Facing::Left => 4,
        Facing::UpLeft => 5,
        Facing::Up => 6,
        Facing::UpRight => 7,
    }
}

fn facing_from_vector(v: Vec2) -> Facing {
    let deg = v.y.atan2(v.x).to_degrees();
    let deg = if deg < 0.0 { deg + 360.0 } else { deg };
    match ((deg + 22.5) / 45.0).floor() as i32 % 8 {
        0 => Facing::Right,
        1 => Facing::UpRight,
        2 => Facing::Up,
        3 => Facing::UpLeft,
        4 => Facing::Left,
        5 => Facing::DownLeft,
        6 => Facing::Down,
        _ => Facing::DownRight,
    }
}