extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

struct Game {
    gl: GlGraphics,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics;
        
        let BG_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(arg.viewport(), | _c, gl| {
            graphics::clear(BG_COLOR, gl);
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window : GlutinWindow = WindowSettings::new(
        "Snake Game",
        [200, 200]
    ).opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl)
    };

    /* Add event loop */
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
    }
}
