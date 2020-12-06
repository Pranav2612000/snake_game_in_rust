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
    food: Food,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics;
        
        let BG_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0]; // white

        self.gl.draw(arg.viewport(), | _c, gl| {
            graphics::clear(BG_COLOR, gl);
        });

        self.snake.render(&mut self.gl, arg);
        self.food.render(&mut self.gl, arg);
    }

    fn update(&mut self) {

        let tail = self.snake.body.back().expect("Snake has no body").clone();
        if(self.food.posX == tail.0.abs() && self.food.posY == tail.1.abs()) {
            println!("Food eaten");
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
        use graphics;
        
        let SNAKE_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0]; //black

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
                    graphics::rectangle(SNAKE_COLOR, square, transform, gl);
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
    posX : i32,
    posY : i32 
}

impl Food {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        let FOOD_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0]; // black

        let food = graphics::rectangle::square(
            (self.posX * 20) as f64,
            (self.posY * 20) as f64,
            20_f64
        );
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(FOOD_COLOR, food, transform, gl);
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
            posX: 3,
            posY: 3
        }
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
