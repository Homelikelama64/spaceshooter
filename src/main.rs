use background::*;
use bullets::*;
use debug::*;
use enemy::*;
use particals::*;
use player::*;
use powerups::*;
use rand::Rng;
use raylib::prelude::*;
use slotmap::{new_key_type, SlotMap};
use ui::*;
use waves::*;

mod background;
mod bullets;
mod debug;
mod enemy;
mod particals;
mod player;
mod powerups;
mod ui;
mod waves;

new_key_type! {
    struct TextureID;
}

#[derive(Clone)]
struct Player {
    pos: Vector2,
    vel: Vector2,
    dir: Vector2,
    speed_original: f32,
    left_turn_original: f32,
    right_turn_original: f32,
    parts: Vec<Part>,
    damage: Vec<Damage>,
    partical_emmiters: Vec<ParticalEmitter>,
    bullet_emmiters: Vec<BulletEmitter>,
    speed: f32,
    left_turn: f32,
    right_turn: f32,
}
#[derive(Clone)]
struct Part {
    pos: Vector2,
    location: Vector2,
    health: f32,
    starting_health: f32,
    size: f32,
    name: String,
}
#[derive(Clone)]
struct Enemy {
    name: String,
    pos: Vector2,
    vel: Vector2,
    dir: Vector2,
    targetpos: Vector2,
    speed: f32,
    turningspeed: f32,
    predictive: bool,
    texture_scale: f32,
    friction: f32,
    size: f32,
    health: f32,
    partical_emmiters: Vec<ParticalEmitter>,
    bullet_emmiters: Vec<BulletEmitter>,
    texture_id: TextureID,
    extra_texture_ids: Vec<TextureID>,
}
#[derive(Clone)]
struct Bullet {
    pos: Vector2,
    vel: Vector2,
    size: f32,
    damage: f32,
    friendly: bool,
    duration: f32,
    time: f32,
}
#[derive(Clone)]
struct Partical {
    pos: Vector2,
    vel: Vector2,
    size: f32,
    shape: ParticalShape,
    starting_color: Color,
    ending_color: Color,
    duration: f32,
    time: f32,
}
#[derive(Clone)]
struct ParticalEmitter {
    pos: Vector2,
    location: Vector2,
    speed_orginal: f32,
    vel: Vector2,
    size: f32,
    shape: ParticalShape,
    starting_color: Color,
    ending_color: Color,
    duration: f32,
    partical_interval: f32,
    time: f32,
    speed: f32,
}

#[derive(Clone)]
struct BulletEmitter {
    pos: Vector2,
    location: Vector2,
    size: f32,
    damage: f32,
    friendly: bool,
    duration: f32,
    bullet_interval: f32,
    time: f32,
}

#[derive(Clone)]
struct Damage {
    src: Vec<usize>,
    des: PartMod,
    index: usize,
    damage_type: DamageType,
}

struct PowerUp {
    pos: Vector2,
    power_type: PowerUpType,
    texture: Texture2D,
}

struct Wave {
    interval: f32,
    min_interval: f32,
    double_spawn_chance: f32,
    max_double_spawn_chance: f32,
    time: f32,
    enemy: Enemy,
}

enum PowerUpType {
    Shield,
    Repair,
}

#[derive(Clone)]
enum DamageType {
    Mult,
    Div,
}

#[derive(Clone)]
enum PartMod {
    Partical,
    Gun,
    TurnLeft,
    TurnRight,
    Speed,
}

#[derive(Clone)]
enum ParticalShape {
    Square,
    Circle,
    RotSquare,
}

fn main() {
    let mut debug = false;
    let (mut rl, thread) = raylib::init()
        .size(get_monitor_width(0), get_monitor_physical_height(0))
        .title("Space Game")
        .resizable()
        .fullscreen()
        .build();
    rl.set_exit_key(None);
    let default_font = rl.get_font_default();

    let mut player = init_player();

    let mut enemies: Vec<Enemy> = vec![];

    let mut bullets: Vec<Bullet> = vec![];

    let mut particals: Vec<Partical> = vec![];

    let mut power_ups: Vec<PowerUp> = vec![PowerUp {
        pos: player.pos,
        power_type: PowerUpType::Repair,
        texture: rl.load_texture(&thread, "Images/Repair.png").unwrap(),
    }];

    let mut textures: SlotMap<TextureID, Texture2D> = SlotMap::with_key();
    let mut waves: Vec<Wave> = init_waves(&mut textures, &mut rl, &thread);

    let ship_image = rl.load_texture(&thread, "Images/V1Ship.png").unwrap();
    let enemy_warning_image = rl.load_texture(&thread, "Images/EnemyWarning.png").unwrap();

    let mut playing: bool = true;
    let mut time = 0.0;
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let screenwidth = rl.get_screen_width();
        let screenheight = rl.get_screen_height();
        if playing {
            time += dt;
        }
        if rl.is_key_released(KeyboardKey::KEY_F3) {
            debug = !debug;
        }
        if rl.is_key_released(KeyboardKey::KEY_ESCAPE) {
            playing = !playing;
        }
        if rl.is_key_released(KeyboardKey::KEY_F11) {
            if rl.is_window_fullscreen() {
                rl.toggle_fullscreen();
                rl.set_window_size(get_monitor_width(0) / 2, get_monitor_height(0) / 2);
            } else {
                rl.set_window_size(get_monitor_width(0), get_monitor_height(0));
                rl.toggle_fullscreen();
            }
        }

        if playing {
            update_waves(&mut waves, &player, &mut enemies, dt);
            update_player(
                &mut player,
                &mut enemies,
                &mut bullets,
                &mut particals,
                &rl,
                dt,
            );
            update_bullets(&mut player, &mut bullets, &mut enemies, &mut particals, dt);
            update_enemies(&mut player, &mut enemies, &mut particals, &mut bullets, dt);
        }
        enemies.retain(|enemy| (enemy.health > 0.0));
        bullets.retain(|bullet| bullet.time < bullet.duration);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(10, 10, 10, 255));
        draw_background(&mut d, &player, screenwidth, screenheight);
        if playing {
            update_particals(&mut particals, dt);
        }
        draw_particals(&mut d, &player, &mut particals, screenwidth, screenheight);
        power_ups_update(
            &mut d,
            &mut player,
            &mut power_ups,
            screenwidth,
            screenheight,
        );
        if debug {
            draw_debug_player(&mut d, &player, screenwidth, screenheight);
        }
        draw_player(&mut d, &player, &ship_image, screenwidth, screenheight);
        if debug {
            draw_debug_enemies(&mut d, &player, &enemies, screenwidth, screenheight);
        }
        draw_enemies(
            &mut d,
            &player,
            &enemies,
            &textures,
            &enemy_warning_image,
            screenwidth,
            screenheight,
        );
        draw_bullets(&mut d, &player, &mut bullets, screenwidth, screenheight);
        draw_timer(&mut d, time, screenwidth);
        draw_part_health(&mut d, &player, &default_font, screenwidth);

        if debug {
            draw_debug_text(&mut d, &player, &enemies, &particals, dt);
        }

        for part in &player.parts {
            if part.health <= 0.0 {
                playing = false
            }
        }
    }
}

fn get_2_mut<T>(xs: &mut [T], a: usize, b: usize) -> Option<(&mut T, &mut T)> {
    if a == b || a >= xs.len() || b >= xs.len() {
        return None;
    }
    let ptr = xs.as_mut_ptr();
    unsafe { Some((&mut *ptr.add(a), &mut *ptr.add(b))) }
}

fn colorlerp(starting_color: Color, ending_color: Color, t: f32) -> Color {
    Color::new(
        (starting_color.r as f32 + (ending_color.r as f32 - starting_color.r as f32) * t) as u8,
        (starting_color.g as f32 + (ending_color.g as f32 - starting_color.g as f32) * t) as u8,
        (starting_color.b as f32 + (ending_color.b as f32 - starting_color.b as f32) * t) as u8,
        (starting_color.a as f32 + (ending_color.a as f32 - starting_color.a as f32) * t) as u8,
    )
}

fn vectortoangle(vector: Vector2) -> f32 {
    f32::atan2(vector.y, vector.x)
}

fn angletovector(angle: f32) -> Vector2 {
    Vector2::new(f32::cos(angle), f32::sin(angle))
}

fn rotatevector(vector: Vector2, angle: f32) -> Vector2 {
    Vector2 {
        x: vector.x * f32::cos(angle) - vector.y * f32::sin(angle),
        y: vector.x * f32::sin(angle) + vector.y * f32::cos(angle),
    }
}

fn enemy_dies(pos: Vector2, vel: Vector2, particals: &mut Vec<Partical>) {
    particalexplosion(
        particals,
        pos,
        vel,
        0.0,
        300.0,
        500,
        Color {
            r: 200,
            g: 200,
            b: 50,
            a: 255,
        },
        Color {
            r: 255,
            g: 0,
            b: 0,
            a: 100,
        },
        0.3,
    );
}

fn particalexplosion(
    particals: &mut Vec<Partical>,
    pos: Vector2,
    vel: Vector2,
    force_min: f32,
    force_max: f32,
    amount: usize,
    start_color: Color,
    ending_color: Color,
    duration: f32,
) {
    for _ in 0..amount {
        particals.push(Partical {
            pos: pos,
            vel: vel
                + angletovector(
                    rand::thread_rng().gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                ) * rand::thread_rng().gen_range(force_min..force_max),
            size: 5.0,
            shape: ParticalShape::Square,
            starting_color: start_color,
            ending_color: ending_color,
            duration: duration,
            time: 0.0,
        });
    }
}
