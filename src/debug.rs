use raylib::prelude::*;

use crate::{colorlerp, Enemy, Partical, Player};

pub fn draw_debug_text(
    d: &mut RaylibDrawHandle,
    player: &Player,
    enemies: &Vec<Enemy>,
    particals: &Vec<Partical>,
    dt: f32,
) {
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

pub fn draw_debug_enemies(
    d: &mut RaylibDrawHandle,
    player: &Player,
    enemies: &Vec<Enemy>,
    screenwidth: i32,
    screenheight: i32,
) {
    for enemy in enemies {
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
}

pub fn draw_debug_player(
    d: &mut RaylibDrawHandle,
    player: &Player,
    screenwidth: i32,
    screenheight: i32,
) {
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
