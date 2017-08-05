extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate rand;

use sdl2_window::Sdl2Window as Window;
// use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;

use graphics::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::{ Events, EventSettings, EventLoop };
use piston::input::{Button, GenericEvent, Input, RenderEvent};
use piston::input::keyboard::Key;


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

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Render(args) => {
                gfx.draw(args.viewport(), |c, gfx| {
                    game.render(c.transform, gfx, SCALE);
                    game.dirty = false;
                });
            }

            Input::Press(Button::Keyboard(key)) => {
                game.key_press(key);
                println!("pressed {:?}", key);
            }

            Input::Update(args) => {
                game.update(args.dt);
            }

            _ => {}
        }
    }
}
