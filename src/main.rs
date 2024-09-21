use background::*;
use bullets::*;
use debug::*;
use enemy::*;
use player::*;
use particals::*;
use ui::*;
use perlin2d::PerlinNoise2D;
use powerups::*;
use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use raylib::prelude::*;

mod ui;
mod particals;
mod player;
mod background;
mod bullets;
mod debug;
mod enemy;
mod powerups;

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
}

struct PowerUp {
    pos: Vector2,
    power_type: PowerUpType,
    texture: Texture2D,
}

enum PowerUpType {
    shield,
    repair,
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
    let default_font = rl.get_font_default();

    let mut player = Player {
        pos: Vector2 { x: 50.0, y: 50.0 },
        vel: Vector2 { x: 10.0, y: 0.0 },
        dir: Vector2 { x: 0.0, y: 1.0 },
        speed_original: 250.0,
        left_turn_original: 100.0,
        right_turn_original: 100.0,
        parts: vec![
            Part {
                pos: Vector2::zero(),
                location: Vector2 { x: 17.0, y: -15.0 },
                health: 4.0,
                starting_health: 4.0,
                size: 15.0,
                name: "Left Engine".to_string(),
            },
            Part {
                pos: Vector2::zero(),
                location: Vector2 { x: -17.0, y: -15.0 },
                health: 4.0,
                starting_health: 4.0,
                size: 15.0,
                name: "Right Engine".to_string(),
            },
            Part {
                pos: Vector2::zero(),
                location: Vector2 { x: 0.0, y: 15.0 },
                health: 3.0,
                starting_health: 3.0,
                size: 20.0,
                name: "Main Body".to_string(),
            },
        ],
        damage: vec![
            Damage {
                src: vec![1],
                des: PartMod::TurnLeft,
                index: 0,
            },
            Damage {
                src: vec![0],
                des: PartMod::TurnRight,
                index: 0,
            },
            Damage {
                src: vec![0, 1, 2],
                des: PartMod::Speed,
                index: 0,
            },
            Damage {
                src: vec![0],
                des: PartMod::Partical,
                index: 0,
            },
            Damage {
                src: vec![1],
                des: PartMod::Partical,
                index: 1,
            },
        ],
        partical_emmiters: vec![
            ParticalEmitter {
                pos: Vector2::zero(),
                location: Vector2::new(21.0, -26.0),
                vel: Vector2::zero(),
                speed_orginal: 200.0,
                size: 5.0,
                shape: ParticalShape::Square,
                starting_color: Color {
                    r: 140,
                    g: 255,
                    b: 251,
                    a: 255,
                },
                ending_color: Color {
                    r: 255,
                    g: 0,
                    b: 50,
                    a: 0,
                },
                duration: 1.0,
                partical_interval: 1.0 / 400.0,
                time: 0.0,
                speed: 0.0,
            },
            ParticalEmitter {
                pos: Vector2::zero(),
                location: Vector2::new(-21.0, -26.0),
                vel: Vector2::zero(),
                speed_orginal: 200.0,
                size: 5.0,
                shape: ParticalShape::Square,
                starting_color: Color {
                    r: 140,
                    g: 255,
                    b: 251,
                    a: 255,
                },
                ending_color: Color {
                    r: 255,
                    g: 0,
                    b: 50,
                    a: 0,
                },
                duration: 1.0,
                partical_interval: 1.0 / 400.0,
                time: 0.0,
                speed: 0.0,
            },
        ],
        bullet_emmiters: vec![
            BulletEmitter {
                pos: Vector2::zero(),
                location: Vector2 { x: 17.0, y: 13.0 },
                size: 5.0,
                damage: 2.0,
                friendly: true,
                duration: 2.0,
                bullet_interval: 1.0 / 7.5,
                time: 0.0,
            },
            BulletEmitter {
                pos: Vector2::zero(),
                location: Vector2 { x: -17.0, y: 13.0 },
                size: 5.0,
                damage: 2.0,
                friendly: true,
                duration: 2.0,
                bullet_interval: 1.0 / 5.0,
                time: 1.0 / 7.5 / 2.0,
            },
        ],
        speed: 0.0,
        left_turn: 0.0,
        right_turn: 0.0,
    };

    let mut enemies: Vec<Enemy> = vec![];

    let mut bullets: Vec<Bullet> = vec![];

    let mut particals: Vec<Partical> = vec![];

    let mut power_ups: Vec<PowerUp> = vec![PowerUp {
        pos: player.pos,
        power_type: PowerUpType::repair,
        texture: rl.load_texture(&thread, "Images/Repair.png").unwrap(),
    }];

    let basic_enemy_image = rl.load_texture(&thread, "Images/V1Enemy.png").unwrap();

    let turret_base_enemy_image = rl.load_texture(&thread, "Images/V2EnemyBase.png").unwrap();
    let turret_top_enemy_image = rl
        .load_texture(&thread, "Images/V2EnemyCannon.png")
        .unwrap();

    let ship_image = rl.load_texture(&thread, "Images/V1Ship.png").unwrap();
    let enemy_warning_image = rl.load_texture(&thread, "Images/EnemyWarning.png").unwrap();

    let mut v1_spawn_interval = 6.0;
    let mut v1_spawn_time = 5.0;
    let mut v1_double_spawn_chance = 0.5;

    let mut v2_spawn_interval = 16.0;
    let mut v2_spawn_time = 0.0;
    let mut v2_double_spawn_chance = 0.1;

    let mut playing: bool = true;
    let mut time = 0.0;
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        if playing {
            time += dt;
        }
        if rl.is_key_released(KeyboardKey::KEY_F3) {
            debug = !debug;
        }
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            playing = !playing;
        }

        player.left_turn = player.left_turn_original;
        player.right_turn = player.right_turn_original;
        player.speed = player.speed_original;

        for partical_emmiter in &mut player.partical_emmiters {
            partical_emmiter.speed = partical_emmiter.speed_orginal
        }

        for enemy in &mut enemies {
            for partical_emmiter in &mut enemy.partical_emmiters {
                partical_emmiter.speed = partical_emmiter.speed_orginal
            }
        }

        for damage in &player.damage {
            let mut health = 0.0;
            let mut total_health = 0.0;
            for src in &damage.src {
                health += player.parts[*src].health;
                total_health += player.parts[*src].starting_health;
            }
            let value = health / total_health;
            match damage.des {
                PartMod::Partical => player.partical_emmiters[damage.index].speed *= value,
                PartMod::Gun => player.bullet_emmiters[damage.index].bullet_interval *= value,
                PartMod::TurnLeft => player.left_turn *= value,
                PartMod::TurnRight => player.right_turn *= value,
                PartMod::Speed => player.speed *= value,
            }
        }

        let screenwidth = rl.get_screen_width();
        let screenheight = rl.get_screen_height();
        if playing {
            while v1_spawn_time > v1_spawn_interval {
                let mut amount = 1;
                while rand::thread_rng().gen_range(0.0..1.0)
                    < v1_double_spawn_chance / (amount * amount) as f32
                {
                    amount += 1;
                }
                v1_double_spawn_chance = f32::min(v1_double_spawn_chance + 0.05, 0.7);
                for _ in 0..amount {
                    v1_spawn_time -= v1_spawn_interval;
                    v1_spawn_interval = f32::max(v1_spawn_interval - 0.1, 1.0);
                    enemies.push(Enemy {
                        name: format!("Basic"),
                        pos: player.pos
                            + angletovector(
                                rand::thread_rng()
                                    .gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                            ) * 2000.0,
                        vel: Vector2 { x: 0.0, y: 0.0 },
                        dir: angletovector(
                            rand::thread_rng()
                                .gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                        ),
                        targetpos: Vector2 { x: 200.0, y: 200.0 },
                        speed: 600.0,
                        turningspeed: 100.0,
                        predictive: false,
                        texture_scale: 1.0,
                        friction: 1.0,
                        size: 16.0,
                        health: 1.0,
                        partical_emmiters: vec![ParticalEmitter {
                            pos: Vector2::zero(),
                            location: Vector2 { x: 0.0, y: -13.0 },
                            vel: Vector2::zero(),
                            speed_orginal: 400.0,
                            size: 5.0,
                            shape: ParticalShape::Square,
                            starting_color: Color {
                                r: 255,
                                g: 255,
                                b: 0,
                                a: 255,
                            },
                            ending_color: Color {
                                r: 255,
                                g: 0,
                                b: 50,
                                a: 0,
                            },
                            duration: 1.0,
                            partical_interval: 1.0 / 400.0,
                            time,
                            speed: 0.0,
                        }],
                        bullet_emmiters: vec![],
                    });
                }
            }
            v1_spawn_time += dt;
            while v2_spawn_time > v2_spawn_interval {
                let mut amount = 1;
                while rand::thread_rng().gen_range(0.0..1.0)
                    < v2_double_spawn_chance / (amount * amount) as f32
                {
                    amount += 1;
                }
                v2_double_spawn_chance = f32::min(v2_double_spawn_chance + 0.05, 0.5);
                for _ in 0..amount {
                    v2_spawn_time -= v2_spawn_interval;
                    v2_spawn_interval = f32::max(v2_spawn_interval - 0.1, 7.0);
                    enemies.push(Enemy {
                        name: format!("Turret"),
                        pos: player.pos
                            + angletovector(
                                rand::thread_rng()
                                    .gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                            ) * 2000.0,
                        vel: Vector2 { x: 0.0, y: 0.0 },
                        dir: angletovector(
                            rand::thread_rng()
                                .gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                        ),
                        targetpos: Vector2 { x: 200.0, y: 200.0 },
                        speed: 500.0,
                        turningspeed: 100.0,
                        predictive: false,
                        texture_scale: 1.5,
                        friction: 1.0,
                        size: 24.0,
                        health: 7.0,
                        partical_emmiters: vec![ParticalEmitter {
                            pos: Vector2::zero(),
                            location: Vector2 { x: 0.0, y: -15.0 },
                            vel: Vector2::zero(),
                            speed_orginal: 800.0,
                            size: 10.0,
                            shape: ParticalShape::Square,
                            starting_color: Color {
                                r: 255,
                                g: 255,
                                b: 0,
                                a: 255,
                            },
                            ending_color: Color {
                                r: 255,
                                g: 0,
                                b: 50,
                                a: 0,
                            },
                            duration: 1.0,
                            partical_interval: 1.0 / 400.0,
                            time,
                            speed: 0.0,
                        }],
                        bullet_emmiters: vec![BulletEmitter {
                            pos: Vector2::zero(),
                            location: Vector2 { x: 0.0, y: 10.0 },
                            size: 5.0,
                            damage: 0.3,
                            friendly: false,
                            duration: 2.0,
                            bullet_interval: 1.0 / 2.0,
                            time: 0.0,
                        }],
                    });
                }
            }
            v2_spawn_time += dt;
        }
        if playing {
            if rl.is_key_down(KeyboardKey::KEY_A) {
                player.dir =
                    angletovector(vectortoangle(player.dir) - (player.left_turn.to_radians() * dt));
            }
            if rl.is_key_down(KeyboardKey::KEY_D) {
                player.dir = angletovector(
                    vectortoangle(player.dir) + (player.right_turn.to_radians() * dt),
                );
            }
        }
        if playing {
            player.vel += player.dir.normalized()
                * (player.speed
                    - (player.vel.length()
                        * (2.0 + (player.vel.normalized().dot(player.dir) - 1.0))
                        / 2.0))
                * dt;
        }
        let right = rotatevector(player.dir, std::f32::consts::PI / 2.0);
        if playing {
            player.vel -= right * (right.dot(player.vel)) * 1.0 * dt;
            player.pos += player.vel * dt;
        }
        for part in &mut player.parts {
            part.pos = player.pos
                + rotatevector(
                    part.location,
                    vectortoangle(player.dir) - std::f32::consts::PI / 2.0,
                )
        }
        for partical_emmiter in &mut player.partical_emmiters {
            partical_emmiter.pos = player.pos
                + rotatevector(
                    partical_emmiter.location,
                    vectortoangle(player.dir) - std::f32::consts::PI / 2.0,
                );
            partical_emmiter.vel = player.vel
                + -player.dir * partical_emmiter.speed * (player.parts[1].health / 2.0)
                + angletovector(
                    rand::thread_rng().gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                ) * rand::thread_rng().gen_range(20.0..40.0);
            if playing {
                while partical_emmiter.time > partical_emmiter.partical_interval {
                    particals.push(Partical {
                        pos: partical_emmiter.pos,
                        vel: partical_emmiter.vel,
                        size: partical_emmiter.size,
                        shape: partical_emmiter.shape.clone(),
                        starting_color: partical_emmiter.starting_color,
                        ending_color: partical_emmiter.ending_color,
                        duration: partical_emmiter.duration,
                        time: 0.0,
                    });
                    partical_emmiter.time -= partical_emmiter.partical_interval;
                }
                partical_emmiter.time += dt;
            }
        }

        let mut fire: bool = false;
        for enemy in &enemies {
            if ((enemy.pos - player.pos).normalized().dot(player.dir) - 1.0).abs() < 0.25 {
                fire = true
            }
        }
        if playing {
            for bullet_emmiter in &mut player.bullet_emmiters {
                bullet_emmiter.pos = player.pos
                    + rotatevector(
                        bullet_emmiter.location,
                        vectortoangle(player.dir) - std::f32::consts::PI / 2.0,
                    );
                let vel = player.vel + player.dir * 500.0;
                while bullet_emmiter.time > bullet_emmiter.bullet_interval {
                    if fire {
                        bullets.push(Bullet {
                            pos: bullet_emmiter.pos,
                            vel: vel,
                            size: bullet_emmiter.size,
                            damage: bullet_emmiter.damage,
                            friendly: bullet_emmiter.friendly,
                            duration: bullet_emmiter.duration,
                            time: 0.0,
                        });
                    }
                    bullet_emmiter.time -= bullet_emmiter.bullet_interval;
                }
                bullet_emmiter.time += dt;
            }
        }
        if playing {
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
        draw_enemies(
            &mut d,
            &player,
            &enemies,
            &basic_enemy_image,
            &turret_base_enemy_image,
            &turret_top_enemy_image,
            &enemy_warning_image,
            screenwidth,
            screenheight,
        );
        if debug {
            draw_debug_enemies(&mut d, &player, &enemies, screenwidth, screenheight);
        }
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
