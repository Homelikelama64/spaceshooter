use raylib::prelude::*;

use crate::{particalexplosion, vectortoangle, Bullet, Enemy, Partical, Player};

pub fn update_bullets(player: &mut Player, bullets: &mut Vec<Bullet>, enemies: &mut Vec<Enemy>,particals: &mut Vec<Partical>,dt:f32) {
    for bullet in bullets {
        bullet.pos += bullet.vel * dt;
        bullet.time += dt;
        if bullet.friendly {
            for enemy in enemies.iter_mut() {
                if bullet.pos.distance_to(enemy.pos) < bullet.size * 2.0 + enemy.size {
                    enemy.health -=
                        bullet.damage - bullet.time / bullet.duration * bullet.damage;
                    particalexplosion(
                        particals,
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
                        particals,
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

pub fn draw_bullets(d:&mut RaylibDrawHandle,player: &Player,bullets: &mut Vec<Bullet>,screenwidth:i32,screenheight:i32) {
    for bullet in bullets {
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
}