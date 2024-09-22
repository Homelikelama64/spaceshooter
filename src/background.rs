use crate::Player;
use perlin2d::PerlinNoise2D;
use rand::prelude::*;
use raylib::prelude::*;

pub fn draw_background(
    d: &mut RaylibDrawHandle,
    player: &Player,
    screenwidth: i32,
    screenheight: i32,
) {
    let scale = 10;
    let noise = PerlinNoise2D::new(1, 1.0, scale as f64 / 5.0, 1.0, 10.0, (1.0, 1.0), 0.0, 0);
    for x in -1..screenwidth / scale + 2 {
        for y in -1..screenheight / scale + 2 {
            let x_screen = x * scale;
            let y_screen = y * scale;
            let x_world = x + player.pos.x as i32 / scale;
            let y_world = y + player.pos.y as i32 / scale;
            let mut rng = StdRng::seed_from_u64((x_world * y_world) as u64 + 1);
            let value = noise.get_noise(x_world as f64, y_world as f64);
            if rng.gen_range(0.0..1.0) < 0.1 {
                d.draw_rectangle_v(
                    Vector2::new(
                        x_screen as f32 - player.pos.x as f32 % scale as f32,
                        y_screen as f32 - player.pos.y as f32 % scale as f32,
                    ) + Vector2::new(
                        rng.gen_range(-scale / 2..scale / 2) as f32,
                        rng.gen_range(-scale / 2..scale / 2) as f32,
                    ),
                    Vector2::new(4.0, 4.0),
                    Color::new(
                        (300.0 * value) as u8,
                        (300.0 * value) as u8,
                        (300.0 * value) as u8,
                        (300.0 * value) as u8,
                    ),
                )
            }
        }
    }
}
