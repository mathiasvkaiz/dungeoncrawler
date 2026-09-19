//! A pixel-rendered sphere. Terrain and flags share the same longitude/latitude
//! transform, so they wrap around the globe and disappear over its horizon.
use bevy::{
    asset::RenderAssetUsages,
    camera::ScalingMode,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    sprite::Anchor,
};
use std::f32::consts::{PI, TAU};

const SIZE: usize = 288;
const ATLAS_W: usize = 720;
const ATLAS_H: usize = 360;
const RADIUS: f32 = 285.0;
const CENTER: Vec2 = Vec2::new(-190.0, -20.0);
const GOLD: Color = Color::srgb(1.0, 0.77, 0.30);
const INK: Color = Color::srgb(0.035, 0.075, 0.09);

pub struct WorldMapPlugin;
impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Flight>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    navigate,
                    paint_planet,
                    update_markers,
                    animate_helicopter,
                    update_panel,
                )
                    .chain(),
            );
        if std::env::args().any(|arg| arg == "--smoke-test") {
            app.add_systems(Update, smoke_test.before(navigate));
        }
    }
}

struct Destination {
    name: &'static str,
    region: &'static str,
    lon: f32,
    lat: f32,
}
const MISSIONS: [Destination; 6] = [
    Destination {
        name: "01 / COASTAL UPRISING",
        region: "WEST AFRICA",
        lon: -12.0,
        lat: 12.0,
    },
    Destination {
        name: "02 / JUNGLE SIGNAL",
        region: "SOUTH AMERICA",
        lon: -60.0,
        lat: -9.0,
    },
    Destination {
        name: "03 / NORTHERN OUTPOST",
        region: "NORTH AMERICA",
        lon: -105.0,
        lat: 40.0,
    },
    Destination {
        name: "04 / DESERT THUNDER",
        region: "CENTRAL ASIA",
        lon: 60.0,
        lat: 35.0,
    },
    Destination {
        name: "05 / ISLAND STRONGHOLD",
        region: "SOUTHEAST ASIA",
        lon: 110.0,
        lat: 8.0,
    },
    Destination {
        name: "06 / SOUTHERN WATCH",
        region: "AUSTRALIA",
        lon: 135.0,
        lat: -25.0,
    },
];

#[derive(Resource)]
struct Flight {
    lon: f32,
    lat: f32,
    selected: usize,
    autopilot: bool,
    briefing: bool,
    velocity: Vec2,
}
impl Default for Flight {
    fn default() -> Self {
        Self {
            lon: -28.0_f32.to_radians(),
            lat: 17.0_f32.to_radians(),
            selected: 0,
            autopilot: false,
            briefing: false,
            velocity: Vec2::ZERO,
        }
    }
}
#[derive(Resource)]
struct Planet {
    image: Handle<Image>,
    atlas: Vec<[u8; 3]>,
    normals: Vec<Option<Vec3>>,
}
#[derive(Component)]
struct Flag(usize);
#[derive(Component)]
struct Chopper;
#[derive(Component)]
struct Rotor;
#[derive(Component)]
struct Panel;

fn rect(commands: &mut Commands, position: Vec3, size: Vec2, color: Color) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(position),
    ));
}
fn label(commands: &mut Commands, value: &str, position: Vec3, size: f32, color: Color) -> Entity {
    commands
        .spawn((
            Text2d::new(value),
            TextFont {
                font_size: FontSize::Px(size),
                ..default()
            },
            TextColor(color),
            Anchor::CENTER_LEFT,
            Transform::from_translation(position),
        ))
        .id()
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: 1280.0,
                min_height: 800.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    // Deterministic stars; no external textures or random dependencies.
    for i in 0..190 {
        let x = ((i * 173 + 37) % 1280) as f32 - 640.0;
        let y = ((i * 293 + 73) % 800) as f32 - 400.0;
        let c = 0.15 + (i % 5) as f32 * 0.035;
        rect(
            &mut commands,
            Vec3::new(x, y, -20.0),
            Vec2::splat(if i % 9 == 0 { 3.0 } else { 1.5 }),
            Color::srgb(c, c * 1.25, c * 1.5),
        );
    }
    label(
        &mut commands,
        "WORLD OPERATIONS",
        Vec3::new(-570.0, 340.0, 20.0),
        32.0,
        GOLD,
    );
    label(
        &mut commands,
        "AIR COMMAND  /  CAMPAIGN NAVIGATION",
        Vec3::new(-570.0, 305.0, 20.0),
        14.0,
        Color::srgb(0.48, 0.66, 0.71),
    );
    rect(
        &mut commands,
        Vec3::new(0.0, 280.0, 20.0),
        Vec2::new(1140.0, 2.0),
        Color::srgb(0.17, 0.29, 0.33),
    );
    rect(
        &mut commands,
        Vec3::new(386.0, -20.0, 15.0),
        Vec2::new(350.0, 540.0),
        Color::srgb(0.045, 0.085, 0.11),
    );
    rect(
        &mut commands,
        Vec3::new(212.0, -20.0, 16.0),
        Vec2::new(3.0, 540.0),
        GOLD,
    );
    let panel = label(
        &mut commands,
        "",
        Vec3::new(238.0, 0.0, 20.0),
        19.0,
        Color::srgb(0.82, 0.89, 0.85),
    );
    commands.entity(panel).insert(Panel);
    label(
        &mut commands,
        "WASD / ARROWS  Fly     SHIFT  Boost     TAB  Next mission     SPACE  Autopilot",
        Vec3::new(-570.0, -345.0, 20.0),
        16.0,
        Color::srgb(0.62, 0.74, 0.77),
    );
    label(
        &mut commands,
        "ENTER  Mission briefing     ESC  Return to map",
        Vec3::new(-570.0, -374.0, 20.0),
        15.0,
        Color::srgb(0.43, 0.56, 0.62),
    );

    let image = images.add(Image::new_fill(
        Extent3d {
            width: SIZE as u32,
            height: SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    ));
    commands.spawn((
        Sprite {
            image: image.clone(),
            custom_size: Some(Vec2::splat(RADIUS * 2.14)),
            ..default()
        },
        Transform::from_translation(CENTER.extend(0.0)),
    ));
    let normals = (0..SIZE * SIZE)
        .map(|i| {
            let x = ((i % SIZE) as f32 + 0.5 - SIZE as f32 / 2.0) / (SIZE as f32 / 2.14);
            let y = -((i / SIZE) as f32 + 0.5 - SIZE as f32 / 2.0) / (SIZE as f32 / 2.14);
            let r = x * x + y * y;
            (r <= 1.0).then(|| Vec3::new(x, y, (1.0 - r).sqrt()))
        })
        .collect();
    commands.insert_resource(Planet {
        image,
        atlas: make_atlas(),
        normals,
    });
    for i in 0..MISSIONS.len() {
        commands
            .spawn((Transform::default(), Visibility::default(), Flag(i)))
            .with_children(|p| {
                p.spawn((
                    Sprite::from_color(INK, Vec2::new(8.0, 34.0)),
                    Transform::from_xyz(0.0, 17.0, 0.0),
                ));
                p.spawn((
                    Sprite::from_color(Color::srgb(0.92, 0.32, 0.20), Vec2::new(31.0, 21.0)),
                    Transform::from_xyz(14.0, 29.0, 0.1),
                ));
                p.spawn((
                    Text2d::new(format!("{}", i + 1)),
                    TextFont {
                        font_size: FontSize::Px(16.0),
                        ..default()
                    },
                    Transform::from_xyz(14.0, 29.0, 0.2),
                ));
            });
    }
    // Pixel-built silhouette with tail, landing skids, cockpit and animated rotor.
    commands
        .spawn((
            Transform::from_xyz(CENTER.x, CENTER.y + 48.0, 12.0),
            Visibility::default(),
            Chopper,
        ))
        .with_children(|p| {
            for (x, y, w, h, color) in [
                (-40.0, 4.0, 48.0, 10.0, INK),
                (-59.0, 14.0, 9.0, 29.0, INK),
                (0.0, 0.0, 65.0, 32.0, INK),
                (4.0, 1.0, 57.0, 25.0, Color::srgb(0.37, 0.43, 0.24)),
                (15.0, 5.0, 24.0, 15.0, Color::srgb(0.36, 0.76, 0.80)),
                (11.0, 6.0, 4.0, 18.0, INK),
                (-9.0, -21.0, 5.0, 18.0, INK),
                (20.0, -21.0, 5.0, 18.0, INK),
                (3.0, -29.0, 66.0, 5.0, Color::srgb(0.62, 0.67, 0.59)),
                (0.0, 24.0, 6.0, 15.0, INK),
                (-14.0, 1.0, 12.0, 5.0, GOLD),
            ] {
                p.spawn((
                    Sprite::from_color(color, Vec2::new(w, h)),
                    Transform::from_xyz(x, y, 0.0),
                ));
            }
            p.spawn((
                Sprite::from_color(Color::srgb(0.74, 0.80, 0.73), Vec2::new(112.0, 4.0)),
                Transform::from_xyz(0.0, 32.0, 1.0),
                Rotor,
            ));
        });
}

fn wrap(angle: f32) -> f32 {
    (angle + PI).rem_euclid(TAU) - PI
}
fn world_point(lon: f32, lat: f32) -> Vec3 {
    Vec3::new(lon.sin() * lat.cos(), lat.sin(), lon.cos() * lat.cos())
}
fn orientation(flight: &Flight) -> Quat {
    Quat::from_rotation_y(flight.lon) * Quat::from_rotation_x(-flight.lat)
}
fn projected(lon: f32, lat: f32, flight: &Flight) -> Vec3 {
    orientation(flight).inverse() * world_point(lon, lat)
}

fn navigate(time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, mut f: ResMut<Flight>) {
    if keys.just_pressed(KeyCode::Escape) {
        f.briefing = false;
    }
    if f.briefing {
        f.velocity = Vec2::ZERO;
        return;
    }
    if keys.just_pressed(KeyCode::Tab) {
        f.selected = (f.selected + 1) % MISSIONS.len();
        f.autopilot = true;
    }
    if keys.just_pressed(KeyCode::Space) {
        f.autopilot = !f.autopilot;
    }
    let axis = |positive, alternate, negative, other| {
        (keys.pressed(positive) || keys.pressed(alternate)) as i32 as f32
            - (keys.pressed(negative) || keys.pressed(other)) as i32 as f32
    };
    let input = Vec2::new(
        axis(
            KeyCode::KeyD,
            KeyCode::ArrowRight,
            KeyCode::KeyA,
            KeyCode::ArrowLeft,
        ),
        axis(
            KeyCode::KeyW,
            KeyCode::ArrowUp,
            KeyCode::KeyS,
            KeyCode::ArrowDown,
        ),
    )
    .normalize_or_zero();
    let dt = time.delta_secs().min(0.05);
    let mission = &MISSIONS[f.selected];
    let delta = Vec2::new(
        wrap(mission.lon.to_radians() - f.lon),
        mission.lat.to_radians() - f.lat,
    );
    if input != Vec2::ZERO {
        f.autopilot = false;
    }
    let target = if f.autopilot {
        delta.clamp_length_max(0.8)
    } else {
        input
            * if keys.pressed(KeyCode::ShiftLeft) {
                1.25
            } else {
                0.55
            }
    };
    f.velocity = f.velocity.lerp(target, 1.0 - (-7.0 * dt).exp());
    f.lon = wrap(f.lon + f.velocity.x * dt);
    f.lat = (f.lat + f.velocity.y * dt).clamp(-1.3, 1.3);
    if f.autopilot && delta.length() < 0.015 {
        f.autopilot = false;
    }
    if keys.just_pressed(KeyCode::Enter) && delta.length() < 0.18 {
        f.briefing = true;
    }
}

fn paint_planet(f: Res<Flight>, planet: Res<Planet>, mut images: ResMut<Assets<Image>>) {
    let Some(mut image) = images.get_mut(&planet.image) else {
        return;
    };
    let Some(data) = image.data.as_mut() else {
        return;
    };
    let rotation = orientation(&f);
    let light = Vec3::new(-0.5, 0.65, 0.8).normalize();
    for (i, pixel) in data.chunks_exact_mut(4).enumerate() {
        if let Some(normal) = planet.normals[i] {
            let world = rotation * normal;
            let u = ((world.x.atan2(world.z) / TAU + 0.5) * ATLAS_W as f32) as usize % ATLAS_W;
            let v = ((0.5 - world.y.clamp(-1.0, 1.0).asin() / PI) * ATLAS_H as f32) as usize;
            let base = planet.atlas[v.min(ATLAS_H - 1) * ATLAS_W + u];
            let shade = (0.35 + 0.65 * normal.dot(light).max(0.0)) * 0.95;
            let rim = (1.0 - normal.z).powi(4) * 0.45;
            for c in 0..3 {
                pixel[c] = (base[c] as f32 * shade + [25.0, 90.0, 120.0][c] * rim).min(255.0) as u8;
            }
            pixel[3] = 255;
        } else {
            let x = ((i % SIZE) as f32 + 0.5 - SIZE as f32 / 2.0) / (SIZE as f32 / 2.14);
            let y = ((i / SIZE) as f32 + 0.5 - SIZE as f32 / 2.0) / (SIZE as f32 / 2.14);
            let glow = ((1.06 - (x * x + y * y).sqrt()) / 0.06).clamp(0.0, 1.0);
            pixel.copy_from_slice(&[47, 144, 183, (glow * glow * 125.0) as u8]);
        }
    }
}

// Optional visual check: capture both hemispheres and a briefing, then exit.
fn smoke_test(
    mut commands: Commands,
    mut frame: Local<u32>,
    mut flight: ResMut<Flight>,
    mut exit: MessageWriter<bevy::app::AppExit>,
) {
    use bevy::render::view::screenshot::{Screenshot, save_to_disk};
    *frame += 1;
    match *frame {
        30 => {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk("target/world-map.png"));
        }
        50 => {
            flight.lon = 135.0_f32.to_radians();
            flight.lat = -25.0_f32.to_radians();
            flight.selected = 5;
        }
        80 => {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk("target/world-map-pacific.png"));
        }
        100 => {
            flight.briefing = true;
        }
        130 => {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk("target/world-map-briefing.png"));
        }
        180 => {
            exit.write(bevy::app::AppExit::Success);
        }
        _ => {}
    }
}

fn update_markers(
    f: Res<Flight>,
    time: Res<Time>,
    mut flags: Query<(&Flag, &mut Transform, &mut Visibility)>,
    mut gizmos: Gizmos,
) {
    for (flag, mut t, mut visible) in &mut flags {
        let m = &MISSIONS[flag.0];
        let p = projected(m.lon.to_radians(), m.lat.to_radians(), &f);
        *visible = if p.z > 0.12 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        t.translation = (CENTER + p.truncate() * RADIUS).extend(3.0 + p.z);
        t.scale = Vec3::splat(0.6 + 0.4 * p.z.max(0.0));
        if flag.0 == f.selected && p.z > 0.12 {
            gizmos.circle_2d(
                CENTER + p.truncate() * RADIUS,
                16.0 + 3.0 * (time.elapsed_secs() * 3.0).sin(),
                GOLD,
            );
        }
    }
    let target = &MISSIONS[f.selected];
    let start = world_point(f.lon, f.lat);
    let end = world_point(target.lon.to_radians(), target.lat.to_radians());
    let arc = Quat::from_rotation_arc(start, end);
    for i in 1..25 {
        let p = orientation(&f).inverse() * (Quat::IDENTITY.slerp(arc, i as f32 / 25.0) * start);
        if p.z > 0.05 {
            gizmos.circle_2d(CENTER + p.truncate() * RADIUS, 1.6, GOLD.with_alpha(0.65));
        }
    }
}
fn animate_helicopter(
    time: Res<Time>,
    f: Res<Flight>,
    mut heli: Query<&mut Transform, (With<Chopper>, Without<Rotor>)>,
    mut rotor: Query<&mut Transform, (With<Rotor>, Without<Chopper>)>,
) {
    for mut t in &mut heli {
        t.translation.y = CENTER.y + 48.0 + (time.elapsed_secs() * 3.0).sin() * 3.0;
        t.rotation = Quat::from_rotation_z(-f.velocity.x * 0.12);
        t.scale.x = if f.velocity.x < -0.03 { -1.0 } else { 1.0 };
    }
    for mut t in &mut rotor {
        t.scale.x = 0.25 + 0.75 * (time.elapsed_secs() * 48.0).cos().abs();
    }
}
fn update_panel(f: Res<Flight>, mut panels: Query<&mut Text2d, With<Panel>>) {
    let m = &MISSIONS[f.selected];
    let near =
        Vec2::new(wrap(m.lon.to_radians() - f.lon), m.lat.to_radians() - f.lat).length() < 0.18;
    let status = if f.briefing {
        "MISSION BRIEFING\n\nSecure the landing zone.\nFind the missing squad.\nSignal for extraction.\n\nMission gameplay is not\nimplemented in this demo.\n\nESC / BACK TO WORLD"
    } else if near {
        "DESTINATION REACHED\n\nENTER / VIEW BRIEFING"
    } else if f.autopilot {
        "EN ROUTE\n\nAutopilot engaged.\nFly manually to take over."
    } else {
        "AWAITING ORDERS\n\nFollow the golden route.\nSPACE / FLY TO TARGET"
    };
    for mut text in &mut panels {
        text.0 = format!(
            "CAMPAIGN / {:02} OF 06\n\n{}\n{}\n\n-------------------------\n\n{}\n\n-------------------------\n\nLAT  {:>6.1}\nLON  {:>6.1}",
            f.selected + 1,
            m.name,
            m.region,
            status,
            f.lat.to_degrees(),
            f.lon.to_degrees()
        );
    }
}

// Original coarse geographic silhouettes, sampled into a reusable color atlas.
fn make_atlas() -> Vec<[u8; 3]> {
    let continents: &[&[(f32, f32)]] = &[
        &[
            (-168., 70.),
            (-145., 72.),
            (-128., 58.),
            (-112., 55.),
            (-95., 70.),
            (-58., 53.),
            (-64., 44.),
            (-82., 25.),
            (-98., 18.),
            (-108., 28.),
            (-123., 40.),
            (-135., 57.),
            (-166., 60.),
        ],
        &[
            (-82., 12.),
            (-65., 10.),
            (-49., 0.),
            (-35., -8.),
            (-42., -23.),
            (-56., -40.),
            (-69., -55.),
            (-76., -30.),
            (-80., -7.),
        ],
        &[
            (-18., 35.),
            (6., 38.),
            (30., 31.),
            (42., 12.),
            (51., 10.),
            (40., -12.),
            (29., -34.),
            (17., -35.),
            (9., -12.),
            (-4., 4.),
            (-17., 15.),
        ],
        &[
            (-10., 36.),
            (-10., 58.),
            (12., 71.),
            (35., 68.),
            (54., 73.),
            (100., 76.),
            (144., 65.),
            (179., 65.),
            (158., 49.),
            (140., 35.),
            (120., 22.),
            (109., 0.),
            (99., 6.),
            (87., 22.),
            (76., 8.),
            (66., 25.),
            (43., 12.),
            (33., 31.),
            (18., 40.),
        ],
        &[
            (113., -22.),
            (122., -15.),
            (136., -12.),
            (143., -10.),
            (154., -27.),
            (146., -39.),
            (130., -32.),
            (114., -35.),
        ],
        &[
            (-52., 59.),
            (-40., 62.),
            (-19., 79.),
            (-40., 83.),
            (-62., 76.),
        ],
        &[(46., -13.), (50., -16.), (47., -26.), (43., -25.)],
        &[(130., 31.), (141., 45.), (145., 42.), (137., 32.)],
        &[(166., -35.), (178., -39.), (170., -47.), (165., -45.)],
    ];
    (0..ATLAS_W * ATLAS_H)
        .map(|i| {
            let lon = (i % ATLAS_W) as f32 / ATLAS_W as f32 * 360.0 - 180.0;
            let lat = 90.0 - (i / ATLAS_W) as f32 / ATLAS_H as f32 * 180.0;
            let rough = (lon * 0.31).sin() * (lat * 0.47).cos() * 1.2;
            let land = continents
                .iter()
                .any(|p| inside(lon + rough, lat + rough, p));
            let noise = ((i.wrapping_mul(1664525).wrapping_add(1013904223) >> 17) % 13) as u8;
            if lat.abs() > 76.0 + 5.0 * (lon * 0.08).sin() {
                [204 + noise, 223 + noise, 219 + noise]
            } else if land {
                if (lat - 23.0).abs() < 12.0 && lon > -20.0 && lon < 80.0 {
                    [186 + noise, 157 + noise, 90 + noise]
                } else if (lon * 0.14 + lat * 0.19).sin() > 0.80 {
                    [96 + noise, 121 + noise, 77 + noise]
                } else if lat.abs() < 15.0 {
                    [47 + noise, 117 + noise, 65 + noise]
                } else {
                    [100 + noise, 153 + noise, 81 + noise]
                }
            } else {
                let coast = continents.iter().any(|p| {
                    inside(lon + 1.5, lat, p)
                        || inside(lon - 1.5, lat, p)
                        || inside(lon, lat + 1.5, p)
                        || inside(lon, lat - 1.5, p)
                });
                if coast {
                    [39, 135, 151]
                } else {
                    [25 + noise / 3, 80 + noise / 2, 121 + noise]
                }
            }
        })
        .collect()
}
fn inside(x: f32, y: f32, points: &[(f32, f32)]) -> bool {
    let mut result = false;
    let mut prev = points[points.len() - 1];
    for &(px, py) in points {
        if (py > y) != (prev.1 > y) && x < (prev.0 - px) * (y - py) / (prev.1 - py) + px {
            result = !result;
        }
        prev = (px, py);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn autopilot_arrives_and_enter_opens_briefing() {
        let mut world = World::new();
        world.insert_resource(Flight {
            lon: -175.0_f32.to_radians(),
            lat: 1.0,
            selected: 5,
            autopilot: true,
            ..default()
        });
        world.insert_resource(Time::<()>::default());
        world.insert_resource(ButtonInput::<KeyCode>::default());
        for _ in 0..1200 {
            world
                .resource_mut::<Time>()
                .advance_by(std::time::Duration::from_secs_f32(1.0 / 60.0));
            world.run_system_once(navigate).unwrap();
        }
        let flight = world.resource::<Flight>();
        assert!(!flight.autopilot);
        assert!(wrap(flight.lon - MISSIONS[5].lon.to_radians()).abs() < 0.03);
        assert!((flight.lat - MISSIONS[5].lat.to_radians()).abs() < 0.03);
        world
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Enter);
        world.run_system_once(navigate).unwrap();
        assert!(world.resource::<Flight>().briefing);
        world.resource_mut::<ButtonInput<KeyCode>>().clear();
        world
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Escape);
        world.run_system_once(navigate).unwrap();
        assert!(!world.resource::<Flight>().briefing);
    }
    #[test]
    fn flight_position_projects_to_front_and_antipode_to_back() {
        for (lon, lat) in [(0.0, 0.0), (2.8, 0.8), (-3.1, -1.2)] {
            let f = Flight {
                lon,
                lat,
                ..default()
            };
            assert!(projected(lon, lat, &f).distance(Vec3::Z) < 0.00001);
            assert!(projected(wrap(lon + PI), -lat, &f).distance(-Vec3::Z) < 0.00001);
        }
    }
    #[test]
    fn shortest_route_crosses_dateline() {
        assert!(
            (wrap((-179.0_f32).to_radians() - 179.0_f32.to_radians()).to_degrees() - 2.0).abs()
                < 0.001
        );
    }
}
