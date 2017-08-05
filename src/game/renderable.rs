use graphics::*;
use opengl_graphics::{ GlGraphics, OpenGL };

use super::Game;
use super::snake::Snake;
use super::map::Map;

pub trait Renderable {
    fn render(&self, t: math::Matrix2d, gfx: &mut GlGraphics, scale: u32);
}

impl Renderable for Game {
    fn render(&self, t: math::Matrix2d, gfx: &mut GlGraphics, scale: u32) {
        clear(color::hex("000000"), gfx);

        self.snake.render(t, gfx, scale);
        self.map.render(t, gfx, scale);
    }
}

impl Renderable for Snake {
    fn render(&self, t: math::Matrix2d, gfx: &mut GlGraphics, scale: u32) {
        for p in self.get_tail().iter() {
            rectangle(color::hex("688f4e"),
                      rectangle::square(p.x as f64 * scale as f64 + 0.05*scale as f64,
                                        p.y as f64 * scale as f64 + 0.05*scale as f64,
                                        scale as f64 * 0.9),
                      t, gfx
            );
        }
    }
}

impl Renderable for Map {
    fn render(&self, t: math::Matrix2d, gfx: &mut GlGraphics, scale: u32) {
        for p in self.get_walls().iter() {
            rectangle(color::hex("cccccc"),
                      rectangle::square(p.x as f64 * scale as f64, p.y as f64 * scale as f64, scale as f64),
                      t, gfx
            );
        }

        let p = self.get_food();
        rectangle(color::hex("ee2222"),
                  rectangle::square(p.x as f64 * scale as f64 + 0.2*scale as f64,
                                    p.y as f64 * scale as f64 + 0.2*scale as f64,
                                    scale as f64 * 0.6),
                  t, gfx
        );
    }
}
