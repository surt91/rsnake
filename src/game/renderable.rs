use graphics::*;
use opengl_graphics::{ GlGraphics, OpenGL };

use super::Game;
use super::snake::Snake;

pub trait Renderable {
    fn render(&self, t: math::Matrix2d, gfx: &mut GlGraphics, scale: u32);
}

impl Renderable for Game {
    fn render(&self, t: math::Matrix2d, gfx: &mut GlGraphics, scale: u32) {
        clear(color::hex("000000"), gfx);

        self.snake.render(t, gfx, scale);
        // TODO render food and walls
    }
}

impl Renderable for Snake {
    fn render(&self, t: math::Matrix2d, gfx: &mut GlGraphics, scale: u32) {
        for p in self.get_tail().iter() {
            rectangle(color::hex("688f4e"),
                      rectangle::square(p.x as f64 * scale as f64 * 0.9, p.y as f64 * scale as f64 * 0.9, scale as f64 * 0.9 * 0.9),
                      t, gfx
            );
        }
    }
}
