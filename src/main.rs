extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate rand;

use sdl2_window::Sdl2Window as Window;
// use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;

use opengl_graphics::{GlGraphics, OpenGL, TextureSettings, Filter};
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event_loop::{Events, EventSettings};
use piston::input::{Button, Input};

mod game;
use game::Game;
use game::renderable::Renderable;

const SIZE: (u32, u32) = (20, 20);
const SCALE: u32 = 20;


fn main() {
    let mut window: Window = WindowSettings::new("RSnake", [SIZE.0 * SCALE, SIZE.1 * SCALE])
                                            .exit_on_esc(true)
                                            .build()
                                            .unwrap();

    let mut gfx = GlGraphics::new(OpenGL::V3_2);

    let mut game = Game::new(SIZE);

    game.print_help();

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", texture_settings)
                                .expect("Could not load font");

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Render(args) => {
                gfx.draw(args.viewport(), |c, gfx| {
                    game.render(c, gfx, SIZE, SCALE, &mut glyphs);
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
