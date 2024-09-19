use std::{borrow::Borrow, time};

use rand::Rng;
use raylib::prelude::*;

const PLAYERSPEED: f32 = 250.0;
const PLAYERTURNSPEED: f32 = 100.0;
const PLAYERSIZE: f32 = 30.0;

struct Player {
    pos: Vector2,
    vel: Vector2,
    dir: Vector2,
    parts: Vec<Part>,
    time_since_last_exaust_1: f32,
    time_since_last_exaust_2: f32,
    time_since_last_bullet: f32,
}

struct Part {
    pos: Vector2,
    location: Vector2,
    health: f32,
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
    time_since_last_exaust: f32,
    exaust_rate: f32,
}

struct Bullet {
    pos: Vector2,
    vel: Vector2,
    size: f32,
    friendly: bool,
    duration: f32,
    time: f32,
}

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

enum ParticalShape {
    Square,
    Circle,
    RotSquare,
}

fn main() {
    let debug = true;
    let (mut rl, thread) = raylib::init()
        .size(1090, 720)
        .title("Hello, World")
        .resizable()
        .build();

    let mut player = Player {
        pos: Vector2 { x: 50.0, y: 50.0 },
        vel: Vector2 { x: 10.0, y: 0.0 },
        dir: Vector2 { x: 0.0, y: 1.0 },
        parts: vec![
            Part {
                pos: Vector2::zero(),
                location: Vector2 { x: 17.0, y: -15.0 },
                health: 2.0,
                size: 15.0,
            },
            Part {
                pos: Vector2::zero(),
                location: Vector2 { x: -17.0, y: -15.0 },
                health: 2.0,
                size: 15.0,
            },
            Part {
                pos: Vector2::zero(),
                location: Vector2 { x: 0.0, y: 15.0 },
                health: 1.0,
                size: 20.0,
            },
        ],
        time_since_last_exaust_1: 0.0,
        time_since_last_exaust_2: 0.0,
        time_since_last_bullet: 0.0,
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
                    time_since_last_exaust: 0.0,
                    exaust_rate: 1.0 / 400.0,
                });
            }
            v1_spawn_time += dt;
        }
        if pause {
            if rl.is_key_down(KeyboardKey::KEY_A) {
                player.dir = angletovector(
                    vectortoangle(player.dir)
                        - (PLAYERTURNSPEED.to_radians() * (player.parts[1].health / 2.0) * dt),
                );
            }
            if rl.is_key_down(KeyboardKey::KEY_D) {
                player.dir = angletovector(
                    vectortoangle(player.dir)
                        + (PLAYERTURNSPEED.to_radians() * (player.parts[0].health / 2.0) * dt),
                );
            }
        }
        if pause {
            player.vel += player.dir.normalized()
                * (PLAYERSPEED
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
        if pause {
            let exaust_rate_1: f32 = 1.0 / 400.0 * (player.parts[1].health / 2.0);
            while player.time_since_last_exaust_1 > exaust_rate_1 {
                particals.push(Partical {
                    pos: player.pos - player.dir * 26.0 + right * 21.0,
                    vel: player.vel
                        + -player.dir * 400.0 * (player.parts[1].health / 2.0)
                        + angletovector(
                            rand::thread_rng()
                                .gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                        ) * rand::thread_rng().gen_range(20.0..40.0),
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
                    time: 0.0,
                });
                player.time_since_last_exaust_1 -= exaust_rate_1;
            }
            let exaust_rate_2: f32 = 1.0 / 400.0 * (player.parts[0].health / 2.0);
            while player.time_since_last_exaust_2 > exaust_rate_2 {
                particals.push(Partical {
                    pos: player.pos - player.dir * 26.0 - right * 21.0,
                    vel: player.vel
                        + -player.dir * 400.0 * (player.parts[0].health / 2.0)
                        + angletovector(
                            rand::thread_rng()
                                .gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                        ) * rand::thread_rng().gen_range(20.0..60.0),
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
                    time: 0.0,
                });
                player.time_since_last_exaust_2 -= exaust_rate_2;
            }
            player.time_since_last_exaust_1 += dt;
            player.time_since_last_exaust_2 += dt;
        }
        let mut fire: bool = false;
        for enemy in &enemies {
            if ((enemy.pos - player.pos).normalized().dot(player.dir) - 1.0).abs() < 0.25 {
                fire = true
            }
        }
        if pause {
            let bullet_rate: f32 = 1.0 / 10.0;
            while player.time_since_last_bullet > bullet_rate {
                if fire {
                    bullets.push(Bullet {
                        pos: player.pos + player.dir * 10.0 + right * 14.0,
                        vel: player.vel + player.dir * 500.0,
                        size: 5.0,
                        friendly: true,
                        duration: 2.0,
                        time: 0.0,
                    });
                    bullets.push(Bullet {
                        pos: player.pos + player.dir * 10.0 - right * 14.0,
                        vel: player.vel + player.dir * 500.0,
                        size: 5.0,
                        friendly: true,
                        duration: 2.0,
                        time: 0.0,
                    });
                }
                player.time_since_last_bullet -= bullet_rate
            }
            player.time_since_last_bullet += dt;
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

                while enemy.time_since_last_exaust > enemy.exaust_rate {
                    particals.push(Partical {
                        pos: enemy.pos - enemy.dir * 13.0 + right * 0.0,
                        vel: enemy.vel
                            + -enemy.dir * 200.0
                            + angletovector(
                                rand::thread_rng()
                                    .gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                            ) * rand::thread_rng().gen_range(20.0..40.0),
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
                        time: 0.0,
                    });
                    enemy.time_since_last_exaust -= enemy.exaust_rate;
                }
                enemy.time_since_last_exaust += dt;
                for part in &mut player.parts {
                    if enemy.pos.distance_to(part.pos) < part.size + enemy.size {
                        enemy.health = -1.0;
                        particals = enemy_dies(enemy.pos, enemy.vel, particals);
                        part.health -= 1.0;
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

        d.clear_background(Color::new(51, 51, 51, 255));

        let size = 20;
        for x in 0..=screenwidth / size {
            let pos = (-player.pos.x % size as f32) as i32 + (x * size);
            d.draw_line(pos, 0, pos, screenheight, Color::BLACK)
        }
        for y in 0..=screenheight / size {
            let pos = (-player.pos.y % size as f32) as i32 + (y * size);
            d.draw_line(0, pos, screenwidth, pos, Color::BLACK)
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

        d.draw_text(
            format!("Pos: {:.2}, {:.2}", player.pos.x, player.pos.y).as_str(),
            5,
            10,
            18,
            Color::WHITE,
        );
        d.draw_text(
            format!("Vel: {:.1}", player.vel.length()).as_str(),
            5,
            30,
            18,
            Color::WHITE,
        );
        d.draw_text(
            format!("Dir: {:.2}, {:.2}", player.dir.x, player.dir.y).as_str(),
            5,
            50,
            18,
            Color::WHITE,
        );
        d.draw_text(
            format!("FPS: {:.2}", 1.0 / dt).as_str(),
            5,
            70,
            18,
            Color::WHITE,
        );
        d.draw_text(
            format!("Particals: {}", particals.len()).as_str(),
            5,
            90,
            18,
            Color::WHITE,
        );

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
