use rand::Rng;

use crate::{
    angletovector, enemy_dies, get_2_mut, particalexplosion, rotatevector, vectortoangle, Bullet,
    Enemy, Partical, Player,
};
use raylib::prelude::*;

pub fn draw_player(
    d: &mut RaylibDrawHandle,
    player: &Player,
    ship_image: &Texture2D,
    screenwidth: i32,
    screenheight: i32,
) {
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
}
