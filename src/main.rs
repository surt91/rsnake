extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;

use opengl_graphics::{GlGraphics, OpenGL, TextureSettings, Filter};
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event_loop::{Events, EventSettings};
use piston::input::{Button, Input};

mod game;
use game::Game;
use game::renderable::Renderable;

mod parse_cl;

fn main() {
    let o = parse_cl::parse_cl();

    let mut window: Window = WindowSettings::new("RSnake", [o.size.0 * o.scale, o.size.1 * o.scale])
                                            .exit_on_esc(true)
                                            .decorated(false)
                                            .srgb(false)
                                            .build()
                                            .unwrap();

    let mut gfx = GlGraphics::new(OpenGL::V3_2);

    let mut game = Game::new(o.size);

    game.print_help();

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", texture_settings)
                                .expect("Could not load font");

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Render(args) => {
                gfx.draw(args.viewport(), |c, gfx| {
                    game.render(c, gfx, o.size, o.scale, &mut glyphs);
                    game.dirty = false;
                });
            }

            Input::Press(Button::Keyboard(key)) => {
                game.key_press(key);
            }

            Input::Update(args) => {
                game.update(args.dt);
            }

            _ => {}
        }
    }
}
