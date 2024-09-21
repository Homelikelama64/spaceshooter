use enemy::*;
use background::*;
use perlin2d::PerlinNoise2D;
use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use raylib::prelude::*;

mod enemy;
mod background;

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
        .size(1020, 720)
        .title("Space Game")
        .resizable()
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
    }];

    let basic_enemy_image = rl.load_texture(&thread, "Images/V1Enemy.png").unwrap();

    let turret_base_enemy_image = rl.load_texture(&thread, "Images/V2EnemyBase.png").unwrap();
    let turret_top_enemy_image = rl
        .load_texture(&thread, "Images/V2EnemyCannon.png")
        .unwrap();

    let ship_image = rl.load_texture(&thread, "Images/V1Ship.png").unwrap();
    let enemy_warning_image = rl.load_texture(&thread, "Images/EnemyWarning.png").unwrap();

    let repair_image = rl.load_texture(&thread, "Images/Repair.png").unwrap();

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
                            damage: 0.1,
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
            update_enemies(&mut player, &mut enemies, &mut particals, &mut bullets, dt);
        }
        if playing {
            for bullet in &mut bullets {
                bullet.pos += bullet.vel * dt;
                bullet.time += dt;
                if bullet.friendly {
                    for enemy in &mut enemies {
                        if bullet.pos.distance_to(enemy.pos) < bullet.size * 2.0 + enemy.size {
                            enemy.health -=
                                bullet.damage - bullet.time / bullet.duration * bullet.damage;
                            particalexplosion(
                                &mut particals,
                                bullet.pos,
                                player.vel,
                                0.0,
                                600.0,
                                50,
                                Color {
                                    r: 255,
                                    g: 0,
                                    b: 0,
                                    a: 255,
                                },
                                Color {
                                    r: 255,
                                    g: 255,
                                    b: 50,
                                    a: 0,
                                },
                                0.1,
                            );
                        }
                    }
                }
                if !bullet.friendly {
                    for part in &mut player.parts {
                        if bullet.pos.distance_to(part.pos) < bullet.size * 2.0 + part.size {
                            part.health -=
                                bullet.damage - bullet.time / bullet.duration * bullet.damage;
                            particalexplosion(
                                &mut particals,
                                bullet.pos,
                                player.vel,
                                0.0,
                                600.0,
                                50,
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
                                0.1,
                            )
                        }
                    }
                }
            }
        }
        enemies.retain(|enemy| (enemy.health > 0.0));
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

        draw_background(&mut d, &player, screenwidth, screenheight);


        for partical in &mut particals {
            if playing {
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

        for power_up in &mut power_ups {
            for part_index in 0..player.parts.len() {
                let part = &mut player.parts[part_index];
                if part.pos.distance_to(power_up.pos) < part.size + 16.0 {
                    match power_up.power_type {
                        PowerUpType::shield => {}
                        PowerUpType::repair => {
                            for other_part_index in 0..player.parts.len() {
                                let Some((part, other_part)) =
                                    get_2_mut(&mut player.parts, part_index, other_part_index)
                                else {
                                    continue;
                                };
                                part.health = part.starting_health;
                                other_part.health = other_part.starting_health;
                            }
                            power_up.pos = player.pos
                                + angletovector(
                                    rand::thread_rng()
                                        .gen_range(-std::f32::consts::PI..std::f32::consts::PI),
                                ) * rand::thread_rng().gen_range(2000.0..2500.0)
                        }
                    }
                }
            }
            d.draw_texture_v(
                &repair_image,
                power_up.pos - player.pos
                    + Vector2::new(screenwidth as f32 / 2.0, screenheight as f32 / 2.0)
                    - Vector2::new(
                        repair_image.width as f32 / 2.0,
                        repair_image.height as f32 / 2.0,
                    ),
                Color::WHITE,
            );
            if player.pos.distance_to(power_up.pos) > 210.0 {
                d.draw_texture_v(
                    &repair_image,
                    (power_up.pos - player.pos).normalized() * 210.0
                        + Vector2::new(screenwidth as f32 / 2.0, screenheight as f32 / 2.0)
                        - Vector2::new(
                            repair_image.width as f32 / 2.0,
                            repair_image.height as f32 / 2.0,
                        ),
                    Color::WHITE,
                )
            }
        }

        if debug {
            for part in &player.parts {
                d.draw_circle_v(
                    Vector2::new(screenwidth as f32 / 2.0, screenheight as f32 / 2.0) + part.pos
                        - player.pos,
                    part.size,
                    colorlerp(
                        Color::GREEN,
                        Color::BLUE,
                        1.0 - part.health / part.starting_health,
                    ),
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
                d.draw_circle_v(
                    enemy.targetpos - player.pos
                        + Vector2::new(screenwidth as f32 / 2.0, screenheight as f32 / 2.0),
                    10.0,
                    Color::ORANGE,
                );
            }
            let pos = Vector2::new(
                enemy.pos.x - player.pos.x + screenwidth as f32 / 2.0,
                enemy.pos.y - player.pos.y + screenheight as f32 / 2.0,
            );
            let mut image: &Texture2D = &basic_enemy_image;
            if enemy.name == "Basic".to_string() {
                image = &basic_enemy_image
            }
            if enemy.name == "Turret".to_string() {
                image = &turret_base_enemy_image;
            }

            d.draw_texture_pro(
                image,
                Rectangle::new(0.0, 0.0, image.width as f32, image.height as f32),
                Rectangle::new(
                    pos.x,
                    pos.y,
                    image.width as f32 * enemy.texture_scale,
                    image.height as f32 * enemy.texture_scale,
                ),
                Vector2::new(
                    image.width as f32 / 2.0 * enemy.texture_scale,
                    image.height as f32 / 2.0 * enemy.texture_scale,
                ),
                vectortoangle(enemy.dir).to_degrees() + 90.0,
                Color::WHITE,
            );
            if enemy.name == "Turret".to_string() {
                d.draw_texture_pro(
                    &turret_top_enemy_image,
                    Rectangle::new(
                        0.0,
                        0.0,
                        turret_top_enemy_image.width as f32,
                        turret_top_enemy_image.height as f32,
                    ),
                    Rectangle::new(
                        pos.x,
                        pos.y + 1.0,
                        turret_top_enemy_image.width as f32 * enemy.texture_scale,
                        turret_top_enemy_image.height as f32 * enemy.texture_scale,
                    ),
                    Vector2::new(
                        turret_top_enemy_image.width as f32 / 2.0 * enemy.texture_scale,
                        turret_top_enemy_image.height as f32 / 2.0 * enemy.texture_scale,
                    ),
                    vectortoangle((player.pos - enemy.pos).normalized()).to_degrees() + 90.0,
                    Color::WHITE,
                );
            }
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
        let mut longest_name: String = "".to_string();
        for part in &player.parts {
            if part.name.len() > longest_name.len() {
                longest_name = part.name.clone();
            }
        }

        for part_index in 0..player.parts.len() {
            let part = &player.parts[part_index];
            let font_size = 18.0;
            let spacing = 5.0;
            let name_length = longest_name.len() as f32 * spacing;
            d.draw_text_pro(
                &default_font,
                part.name.as_str(),
                Vector2 {
                    x: screenwidth as f32 - name_length as f32 * 3.0,
                    y: 10.0 + (part_index as f32 * (font_size + 15.0)),
                },
                Vector2 { x: 0.0, y: 0.0 },
                0.0,
                font_size,
                spacing,
                Color::WHITE,
            );
            d.draw_text_pro(
                &default_font,
                ("Health: ".to_string()
                    + (part.health / part.starting_health * 100.0)
                        .to_string()
                        .as_str()
                    + "%")
                    .as_str(),
                Vector2 {
                    x: screenwidth as f32 - name_length as f32 * 3.0 + 10.0,
                    y: 28.0 + (part_index as f32 * (font_size + 15.0)),
                },
                Vector2 { x: 0.0, y: 0.0 },
                0.0,
                font_size / 1.1,
                spacing / 1.1,
                Color::WHITE,
            );
        }

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
