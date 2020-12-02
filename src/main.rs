extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone, PartialEq)]
enum Direction {
    Right, Left, Up, Down
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics;
        
        let BG_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0]; // white

        self.gl.draw(arg.viewport(), | _c, gl| {
            graphics::clear(BG_COLOR, gl);
        });

        self.snake.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        self.snake.update();
    }

    fn pressed(&mut self, btn: &Button) {
        let last_dir = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_dir != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_dir != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_dir != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_dir != Direction::Left=> Direction::Right,
            _ => last_dir
        };
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;
        
        let SNAKE_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0]; //black

        let squares : Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                    (x * 20) as f64,
                    (y * 20) as f64, 
                    20_f64
                )
            })
            .collect();


        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares.into_iter()
                .for_each(|square| {
                    graphics::rectangle(SNAKE_COLOR, square, transform, gl);
                });
        });
        
    }

    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();
        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down=> new_head.1 += 1,
        }

        self.body.push_front(new_head);

        self.body.pop_back().unwrap();
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
        gl: GlGraphics::new(opengl),
        snake: Snake { 
            body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
            dir: Direction::Right
        },
    };

    /* Add event loop */
    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
