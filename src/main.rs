use std::{borrow::Borrow, time};

use noise::NoiseFn;
use perlin2d::PerlinNoise2D;
use rand::{rngs::StdRng, Rng, SeedableRng};
use raylib::prelude::*;

const PLAYERSPEED: f32 = 250.0;
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
    speed_orginal:f32,
    vel: Vector2,
    size: f32,
    shape: ParticalShape,
    starting_color: Color,
    ending_color: Color,
    duration: f32,
    partical_interval: f32,
    time: f32,
    speed:f32
}

#[derive(Clone)]
struct BulletEmitter {
    pos: Vector2,
    location: Vector2,
    size: f32,
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
        .size(1090, 720)
        .title("Hello, World")
        .resizable()
        .build();

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
                health: 2.0,
                starting_health: 2.0,
                size: 15.0,
            },
            Part {
                pos: Vector2::zero(),
                location: Vector2 { x: -17.0, y: -15.0 },
                health: 2.0,
                starting_health: 2.0,
                size: 15.0,
            },
            Part {
                pos: Vector2::zero(),
                location: Vector2 { x: 0.0, y: 15.0 },
                health: 1.0,
                starting_health: 2.0,
                size: 20.0,
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
                src: vec![0,1],
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
                speed_orginal:400.0,
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
                speed_orginal:400.0,
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
                friendly: true,
                duration: 2.0,
                bullet_interval: 1.0 / 7.5,
                time: 0.0,
            },
            BulletEmitter {
                pos: Vector2::zero(),
                location: Vector2 { x: -17.0, y: 13.0 },
                size: 5.0,
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

    let basic_enemy_image = rl.load_texture(&thread, "Images/V1Enemy.png").unwrap();
    let ship_image = rl.load_texture(&thread, "Images/V1Ship.png").unwrap();
    let enemy_warning_image = rl.load_texture(&thread, "Images/EnemyWarning.png").unwrap();

    let mut v1_spawn_interval = 6.0;
    let mut v1_spawn_time = 0.0;

    let mut pause: bool = true;
    let mut time = 0.0;
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        if pause {
            time += dt;
        }
        if rl.is_key_released(KeyboardKey::KEY_F3) {
            debug = !debug;
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
        if pause {
            while v1_spawn_time > v1_spawn_interval {
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
                    dir: Vector2 { x: 0.0, y: 1.0 },
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
                        speed_orginal:400.0,
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
            v1_spawn_time += dt;
        }
        if pause {
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
        if pause {
            player.vel += player.dir.normalized()
                * (player.speed
                    - (player.vel.length()
                        * (2.0 + (player.vel.normalized().dot(player.dir) - 1.0))
                        / 2.0))
                * dt;
        }
        let right = rotatevector(player.dir, std::f32::consts::PI / 2.0);
        if pause {
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
            if pause {
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
        if pause {
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
        for enemy_index in 0..enemies.len() {
            let enemy = &mut enemies[enemy_index];
            if enemy.predictive {
                let mut time_to_reach = 0.0;
                for _ in 0..10 {
                    enemy.targetpos = player.pos + player.dir * player.vel.length() * time_to_reach;
                    time_to_reach = enemy.targetpos.distance_to(enemy.pos) / enemy.vel.length();
                }
            } else {
                enemy.targetpos = player.pos
            }
            let right = rotatevector(enemy.dir, std::f32::consts::PI / 2.0);
            if right.dot(enemy.targetpos - enemy.pos) > 0.0 {
                enemy.dir =
                    angletovector(vectortoangle(enemy.dir) + (enemy.turningspeed.to_radians() * dt))
            } else {
                enemy.dir =
                    angletovector(vectortoangle(enemy.dir) - (enemy.turningspeed.to_radians() * dt))
            }
            if pause {
                enemy.vel += enemy.dir.normalized()
                    * (enemy.speed
                        - (enemy.vel.length()
                            * (2.0 + (enemy.vel.normalized().dot(enemy.dir) - 1.0))
                            / 2.0))
                    * dt;

                enemy.vel -= right * (right.dot(enemy.vel)) * enemy.friction * dt;

                enemy.pos += enemy.vel * dt;

                for partical_emmiter in &mut enemy.partical_emmiters {
                    partical_emmiter.pos = enemy.pos
                        + rotatevector(
                            partical_emmiter.location,
                            vectortoangle(enemy.dir) - std::f32::consts::PI / 2.0,
                        );
                    partical_emmiter.vel = enemy.vel
                        + -enemy.dir * partical_emmiter.speed
                        + angletovector(
                            rand::thread_rng()
                                .gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                        ) * rand::thread_rng().gen_range(20.0..40.0);
                    if pause {
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
                for part in &mut player.parts {
                    if enemy.pos.distance_to(part.pos) < part.size + enemy.size {
                        enemy.health = -1.0;
                        particals = enemy_dies(enemy.pos, enemy.vel, particals);
                        part.health -= 1.0;
                        particals = particalexplosion(
                            particals,
                            part.pos,
                            player.vel,
                            0.0,
                            300.0,
                            500,
                            Color {
                                r: 140,
                                g: 255,
                                b: 251,
                                a: 255,
                            },
                            Color {
                                r: 255,
                                g: 0,
                                b: 50,
                                a: 0,
                            },
                            1.0,
                        );
                    }
                }
            }
            bullets.retain(|bullet| {
                bullet.pos.distance_to(enemy.pos) > bullet.size * 2.0 + enemy.size
            });
            if pause {
                for other_enemy_index in 0..enemies.len() {
                    let Some((enemy, other_enemy)) =
                        get_2_mut(&mut enemies, enemy_index, other_enemy_index)
                    else {
                        continue;
                    };
                    if enemy.pos.distance_to(other_enemy.pos) < other_enemy.size + enemy.size {
                        enemy.health = -1.0;
                        other_enemy.health = -1.0;
                        particals = enemy_dies(enemy.pos, enemy.vel, particals);
                        particals = enemy_dies(other_enemy.pos, other_enemy.vel, particals);
                    };
                }
            }
        }
        enemies.retain(|enemy| (enemy.health > 0.0));
        if pause {
            for bullet in &mut bullets {
                bullet.pos += bullet.vel * dt;
                bullet.time += dt;
                for enemy in &mut enemies {
                    if bullet.pos.distance_to(enemy.pos) < bullet.size * 2.0 + enemy.size {
                        enemy.health -= 1.0 - bullet.time / bullet.duration;
                        if enemy.health <= 0.0 {
                            particals = enemy_dies(enemy.pos, enemy.vel, particals)
                        }
                    }
                }
            }
        }
        bullets.retain(|bullet| bullet.time < bullet.duration);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(10, 10, 10, 255));

        //let size = 20;
        //for x in 0..=screenwidth / size {
        //    let pos = (-player.pos.x % size as f32) as i32 + (x * size);
        //    d.draw_line(pos, 0, pos, screenheight, Color::BLACK)
        //}
        //for y in 0..=screenheight / size {
        //    let pos = (-player.pos.y % size as f32) as i32 + (y * size);
        //    d.draw_line(0, pos, screenwidth, pos, Color::BLACK)
        //}

        let background_noise =
            PerlinNoise2D::new(1, 1.0, screenwidth as f64, 1.0, 2.0, (1.0, 1.0), 1.0, 1);
        let scale = 10;
        for x in 0..screenwidth / scale {
            for y in 0..screenheight / scale {
                let x_screen = x * scale;
                let y_screen = y * scale;
                let x_world = x + player.pos.x as i32 / scale;
                let y_world = y + player.pos.y as i32 / scale;
                let value = background_noise.get_noise(x_world as f64, y_world as f64);
                let mut rng = StdRng::seed_from_u64((x_world * y_world) as u64);
                if rng.gen_range(0.0..1.0).clone() > value {
                    d.draw_rectangle_v(
                        Vector2::new(
                            x_screen as f32 - player.pos.x % scale as f32,
                            y_screen as f32 - player.pos.y % scale as f32 as f32,
                        ) + Vector2::new(
                            rng.gen_range(-scale / 2..scale / 2) as f32,
                            rng.gen_range(-scale / 2..scale / 2) as f32,
                        ),
                        Vector2::new(4.0, 4.0),
                        Color::new(
                            (255.0 * value) as u8,
                            (255.0 * value) as u8,
                            (255.0 * value) as u8,
                            (255.0 * value) as u8,
                        ),
                    )
                }
            }
        }

        for partical in &mut particals {
            if pause {
                partical.pos += partical.vel * dt;
                partical.time += dt;
            }
            let lerped_color = colorlerp(
                partical.starting_color,
                partical.ending_color,
                partical.time / partical.duration,
            );
            match partical.shape {
                ParticalShape::Square => d.draw_rectangle_v(
                    Vector2::new(
                        partical.pos.x - player.pos.x + screenwidth as f32 / 2.0
                            - partical.size / 2.0,
                        partical.pos.y - player.pos.y + screenheight as f32 / 2.0
                            - partical.size / 2.0,
                    ),
                    Vector2::new(partical.size, partical.size),
                    lerped_color,
                ),
                ParticalShape::Circle => d.draw_circle_v(
                    Vector2::new(
                        partical.pos.x - player.pos.x + screenwidth as f32 / 2.0,
                        partical.pos.y - player.pos.y + screenheight as f32 / 2.0,
                    ),
                    partical.size / 2.0,
                    lerped_color,
                ),
                ParticalShape::RotSquare => {}
            }
        }
        particals.retain(|partical| partical.time < partical.duration);

        //if debug {
        //    d.draw_circle_v(
        //        Vector2::new(screenwidth as f32 / 2.0, screenheight as f32 / 2.0),
        //        PLAYERSIZE,
        //        Color::GREEN,
        //    );
        //}

        if debug {
            for part in &player.parts {
                d.draw_circle_v(
                    Vector2::new(screenwidth as f32 / 2.0, screenheight as f32 / 2.0) + part.pos
                        - player.pos,
                    part.size,
                    Color::GREEN,
                );
            }
        }

        let ship_scale = 2.0;
        d.draw_texture_pro(
            &ship_image,
            Rectangle::new(0.0, 0.0, ship_image.width as f32, ship_image.height as f32),
            Rectangle::new(
                screenwidth as f32 / 2.0,
                screenheight as f32 / 2.0,
                ship_image.width as f32 * ship_scale,
                ship_image.height as f32 * ship_scale,
            ),
            Vector2::new(
                ship_image.width as f32 / 2.0 * ship_scale,
                ship_image.height as f32 / 2.0 * ship_scale,
            ),
            vectortoangle(player.dir).to_degrees() + 90.0,
            Color::WHITE,
        );

        for enemy in &enemies {
            if debug {
                d.draw_circle_v(
                    enemy.pos - player.pos
                        + Vector2::new(screenwidth as f32 / 2.0, screenheight as f32 / 2.0),
                    enemy.size,
                    Color::RED,
                );
            }
            let pos = Vector2::new(
                enemy.pos.x - player.pos.x + screenwidth as f32 / 2.0,
                enemy.pos.y - player.pos.y + screenheight as f32 / 2.0,
            );
            d.draw_texture_pro(
                &basic_enemy_image,
                Rectangle::new(
                    0.0,
                    0.0,
                    basic_enemy_image.width as f32,
                    basic_enemy_image.height as f32,
                ),
                Rectangle::new(
                    pos.x,
                    pos.y,
                    basic_enemy_image.width as f32 * enemy.texture_scale,
                    basic_enemy_image.height as f32 * enemy.texture_scale,
                ),
                Vector2::new(
                    basic_enemy_image.width as f32 / 2.0 * enemy.texture_scale,
                    basic_enemy_image.height as f32 / 2.0 * enemy.texture_scale,
                ),
                vectortoangle(enemy.dir).to_degrees() + 90.0,
                Color::WHITE,
            );
            if player.pos.distance_to(enemy.pos) > 170.0 {
                d.draw_texture_v(
                    &enemy_warning_image,
                    (enemy.pos - player.pos).normalized() * 170.0
                        + Vector2::new(screenwidth as f32 / 2.0, screenheight as f32 / 2.0)
                        - Vector2::new(
                            enemy_warning_image.width as f32 / 2.0,
                            enemy_warning_image.height as f32 / 2.0,
                        ),
                    Color::WHITE,
                )
            }
        }

        for bullet in &bullets {
            let bullet_scale = 1.0 - bullet.time / bullet.duration;
            let bullet_width = bullet.size * bullet_scale;
            let bullet_length = bullet.size * 2.0 * bullet_scale;
            let mut color = Color::GREEN;
            if !bullet.friendly {
                color = Color::RED
            }
            d.draw_rectangle_pro(
                Rectangle::new(
                    bullet.pos.x - player.pos.x + screenwidth as f32 / 2.0,
                    bullet.pos.y - player.pos.y + screenheight as f32 / 2.0,
                    bullet_width,
                    bullet_length,
                ),
                Vector2::new(bullet_width, bullet_length),
                vectortoangle(bullet.vel).to_degrees() + 90.0,
                color,
            )
        }

        d.draw_text(
            format!("Time: {:.2}", time).as_str(),
            screenwidth / 2 - 50,
            10,
            36,
            Color::WHITE,
        );

        if debug {
            d.draw_text(
                format!("Pos: {:.2}, {:.2}", player.pos.x, player.pos.y).as_str(),
                5,
                10,
                18,
                Color::WHITE,
            );
            d.draw_text(
                format!("Target Speed: {:.1}", player.speed).as_str(),
                5,
                30,
                18,
                Color::WHITE,
            );
            d.draw_text(
                format!("Vel: {:.1}", player.vel.length()).as_str(),
                5,
                50,
                18,
                Color::WHITE,
            );
            d.draw_text(
                format!("Dir: {:.2}, {:.2}", player.dir.x, player.dir.y).as_str(),
                5,
                70,
                18,
                Color::WHITE,
            );
            d.draw_text(
                format!("FPS: {:.2}", 1.0 / dt).as_str(),
                5,
                90,
                18,
                Color::WHITE,
            );
            d.draw_text(
                format!("Particals: {}", particals.len()).as_str(),
                5,
                110,
                18,
                Color::WHITE,
            );
            d.draw_text(
                format!("Enemys: {}", enemies.len()).as_str(),
                5,
                130,
                18,
                Color::WHITE,
            );
            d.draw_text(
                format!("Turning Left: {}", player.left_turn).as_str(),
                5,
                150,
                18,
                Color::WHITE,
            );
            d.draw_text(
                format!("Turning Right: {}", player.right_turn).as_str(),
                5,
                170,
                18,
                Color::WHITE,
            );

        }
        for part in &player.parts {
            if part.health <= 0.0 {
                pause = false
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

fn enemy_dies(pos: Vector2, vel: Vector2, particals: Vec<Partical>) -> Vec<Partical> {
    return particalexplosion(
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
    mut particals: Vec<Partical>,
    pos: Vector2,
    vel: Vector2,
    force_min: f32,
    force_max: f32,
    amount: usize,
    start_color: Color,
    ending_color: Color,
    duration: f32,
) -> Vec<Partical> {
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
    return particals;
}
