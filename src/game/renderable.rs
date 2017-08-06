use std::cmp::max;

use graphics::*;
use graphics::character::CharacterCache;

use super::Game;
use super::snake::Snake;
use super::map::Map;

fn render_text<C, G>(text: &str, font_size: u32, pos: (i32, i32), color: &str, c: Context, gfx: &mut G, glyphs: &mut C)
    where C: CharacterCache, G: Graphics<Texture=C::Texture>
{
    let t = text::Text::new_color(color::hex(color), font_size);

    t.draw(
            text,
            glyphs, &c.draw_state,
            c.transform.trans(pos.0 as f64, pos.1 as f64), gfx
    );
}

fn render_score<C, G>(score: i64, c: Context, gfx: &mut G, size: (u32, u32), scale: u32, glyphs: &mut C)
    where C: CharacterCache, G: Graphics<Texture=C::Texture>
{
    let offset = 20;
    let font_size = 3 * scale as i32;
    let dx = (size.0 * scale) as i32 - offset - max(1, (score as f64 + 0.9).log10().ceil() as i32) * (0.4 * font_size as f64) as i32;
    let dy = (size.1 * scale) as i32 - offset;

    render_text(&format!("{}", score), font_size as u32, (dx, dy), "666666", c, gfx, glyphs)
}

fn render_game_over<C, G>(text: &str, score: i64, c: Context, gfx: &mut G, size: (u32, u32), scale: u32, glyphs: &mut C)
    where C: CharacterCache, G: Graphics<Texture=C::Texture>
{
    let offset = 20;
    let font_size = 3 * scale as i32;

    // FIXME: dx needs to be adjusted properly
    let dx = offset + ((size.0 * scale) as f64 / 2.) as i32 - 3 * font_size;
    let dy = offset + 3 * font_size;

    render_text(text, font_size as u32, (dx, dy), "ee33333", c, gfx, glyphs);

    let dx = dx + (5.8*font_size as f64) as i32 - font_size - offset - (score as f64 + 0.9).log10().ceil() as i32 * font_size;
    let dy = dy + font_size;
    render_text(&format!("{}", score), font_size as u32, (dx, dy), "ee33333", c, gfx, glyphs);
}

fn render_help<C, G>(texts: &[&str], c: Context, gfx: &mut G, size: (u32, u32), scale: u32, glyphs: &mut C)
    where C: CharacterCache, G: Graphics<Texture=C::Texture>
{
    rectangle(color::hex("cccccc"),
              [
                1. * scale as f64,
                1. * scale as f64,
                ((size.0 - 2) * scale) as f64,
                ((size.1 - 2) * scale) as f64,
              ],
              c.transform, gfx
    );

    let offset = 20 + scale as i32;
    let font_size = 2 * scale as i32;

    // FIXME: dx needs to be adjusted properly
    let dx = offset;
    let mut dy = offset + font_size;

    render_text("rsnake", font_size as u32, (dx, dy), "688f4e", c, gfx, glyphs);

    let font_size = scale as i32;
    dy += offset;
    for i in texts {
        dy += (1.1 * font_size as f64) as i32;
        render_text(i, font_size as u32, (dx, dy), "333333", c, gfx, glyphs);
    }
}

pub trait Renderable {
    fn render<C, G>(&self, c: Context, gfx: &mut G, size: (u32, u32), scale: u32, glyphs: &mut C)
        where C: CharacterCache, G: Graphics<Texture=C::Texture>;
}

impl Renderable for Game {
    fn render<C, G>(&self, c: Context, gfx: &mut G, size: (u32, u32), scale: u32, glyphs: &mut C)
        where C: CharacterCache, G: Graphics<Texture=C::Texture>
    {
        clear(color::hex("000000"), gfx);

        // render score
        if !self.game_over {
            render_score(self.score, c, gfx, size, scale, glyphs);
        }

        if self.paused {
            render_help(&self.help_texts, c, gfx, size, scale, glyphs);
        } else {
            // render content
            self.snake.render(c, gfx, size, scale, glyphs);
            self.map.render(c, gfx, size, scale, glyphs);

            // render Game Over
            if self.game_over {
                render_game_over("Game Over!", self.score, c, gfx, size, scale, glyphs);
            }
            if self.game_won {
                render_game_over("You Win!", self.score, c, gfx, size, scale, glyphs);
            }
        }
    }
}

impl Renderable for Snake {
    fn render<C, G>(&self, c: Context, gfx: &mut G, _: (u32, u32), scale: u32, _: &mut C)
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
    fn render<C, G>(&self, c: Context, gfx: &mut G, _: (u32, u32), scale: u32, _: &mut C)
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
