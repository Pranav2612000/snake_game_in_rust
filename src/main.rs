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

use rand::Rng;

#[derive(Clone, PartialEq)]
enum Direction {
    Right, Left, Up, Down
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        let bg_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0]; // white

        self.gl.draw(arg.viewport(), | _c, gl| {
            graphics::clear(bg_color, gl);
        });

        self.snake.render(&mut self.gl, arg);
        self.food.render(&mut self.gl, arg);
    }

    fn update(&mut self) {

        let tail = self.snake.body.back().expect("Snake has no body").clone();
        if self.food.pos_x == tail.0.abs() && self.food.pos_y == tail.1.abs() {
            let mut rng = rand::thread_rng();
            let new_food = Food {
                pos_x: rng.gen_range(0, 10),
                pos_y: rng.gen_range(0, 10) 
            };
            self.food = new_food;
        }

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
        let snake_color: [f32; 4] = [0.0, 0.0, 0.0, 1.0]; //black

        let squares : Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                    (((x * 20) + 200) % 200) as f64,
                    (((y * 20) + 200) % 200) as f64, 
                    20 as f64
                )
            })
            .collect();


        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares.into_iter()
                .for_each(|square| {
                    graphics::rectangle(snake_color, square, transform, gl);
                });
        });
        
    }

    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();
        match self.dir {
            Direction::Left => new_head.0 = ((new_head.0 + 10) - 1) % 10,
            Direction::Right => new_head.0 = ((new_head.0 + 10) + 1) % 10,
            Direction::Up => new_head.1 = ((new_head.1 + 10) - 1) % 10,
            Direction::Down=> new_head.1 = ((new_head.1 + 10) + 1) % 10,
        }

        self.body.push_front(new_head);

        self.body.pop_back().unwrap();
    }
}

struct Food {
    pos_x : i32,
    pos_y : i32 
}

impl Food {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let food_color: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; // red 

        let food = graphics::rectangle::square(
            (self.pos_x * 20) as f64,
            (self.pos_y * 20) as f64,
            20_f64
        );
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(food_color, food, transform, gl);
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
        gl: GlGraphics::new(opengl),
        snake: Snake { 
            body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
            dir: Direction::Right
        },
        food: Food {
            pos_x: 3,
            pos_y: 3
        }
    };

    /* Add event loop */
    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(_) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
