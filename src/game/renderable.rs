use std::cmp::max;

use graphics::*;
use graphics::character::CharacterCache;

use super::Game;
use super::snake::Snake;
use super::map::Map;

pub trait Renderable {
    fn render<C, G>(&self, c: Context, gfx: &mut G, size: (u32, u32), scale: u32, glyphs: &mut C)
        where C: CharacterCache, G: Graphics<Texture=C::Texture>;
}

impl Renderable for Game {
    fn render<C, G>(&self, c: Context, gfx: &mut G, size: (u32, u32), scale: u32, glyphs: &mut C)
        where C: CharacterCache, G: Graphics<Texture=C::Texture>
    {
        clear(color::hex("000000"), gfx);

        let offset = 20;
        let font_size = 3 * scale;
        let t = text::Text::new_color(color::hex("666666"), font_size);
        let dx = size.0 * scale - font_size - offset - (self.score as f64).log10().ceil() as u32;
        let dy = size.1 * scale - offset;
        t.draw(
                &format!("{}", self.score),
                glyphs, &c.draw_state,
                c.transform.trans(dx as f64, dy as f64), gfx
        );

        self.snake.render(c, gfx, size, scale, glyphs);
        self.map.render(c, gfx, size, scale, glyphs);
    }
}

impl Renderable for Snake {
    fn render<C, G>(&self, c: Context, gfx: &mut G, size: (u32, u32), scale: u32, glyphs: &mut C)
        where C: CharacterCache, G: Graphics<Texture=C::Texture>
    {
        let tip = max(1, self.length as i32 - 5) as usize;
        for p in self.get_tail()
                     .iter()
                     .skip(1) // do not paint head
                     .take(tip) // do not paint last 5 segments
        {
            rectangle(color::hex("688f4e"),
                      rectangle::square(p.x as f64 * scale as f64 + 0.05*scale as f64,
                                        p.y as f64 * scale as f64 + 0.05*scale as f64,
                                        scale as f64 * 0.9),
                      c.transform, gfx
            );
        }
        // different head color
        rectangle(color::hex("8db465"),
                  rectangle::square(self.head().x as f64 * scale as f64 + 0.01*scale as f64,
                                    self.head().y as f64 * scale as f64 + 0.01*scale as f64,
                                    scale as f64 * 0.98),
                  c.transform, gfx
        );

        // smaller tail
        for (n, p) in self.get_tail()
                     .iter()
                     .skip(tip) // do not paint head
                     .enumerate()
        {
            let n = n + 1;
            rectangle(color::hex("688f4e"),
                      rectangle::square(p.x as f64 * scale as f64 + 0.05 * n as f64 * scale as f64,
                                        p.y as f64 * scale as f64 + 0.05 * n as f64 * scale as f64,
                                        scale as f64 * (1. - 0.1 * n as f64)),
                      c.transform, gfx
            );
        }
    }
}

impl Renderable for Map {
    fn render<C, G>(&self, c: Context, gfx: &mut G, size: (u32, u32), scale: u32, glyphs: &mut C)
        where C: CharacterCache, G: Graphics<Texture=C::Texture>
    {
        for p in self.get_walls().iter() {
            rectangle(color::hex("cccccc"),
                      rectangle::square(p.x as f64 * scale as f64,
                                        p.y as f64 * scale as f64,
                                        scale as f64),
                      c.transform, gfx
            );
        }

        let p = self.get_food();
        rectangle(color::hex("ee2222"),
                  rectangle::square(p.x as f64 * scale as f64 + 0.2*scale as f64,
                                    p.y as f64 * scale as f64 + 0.2*scale as f64,
                                    scale as f64 * 0.6),
                  c.transform, gfx
        );
    }
}
