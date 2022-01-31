//Made by deca
#[allow(unused_imports)]
use ggez::{ContextBuilder, event::EventHandler, Context, error::GameResult, error::GameError};
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, DrawParam};
use std::collections::LinkedList;
use ggez::input::keyboard::*;
use ggez::mint;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Eat {
    Full,
    Hungry
}

struct Apple {
    color: Color,
    width: i32,
    height: i32,
    pos: (i32, i32)
}

struct Part {
    x: i32,
    y: i32,
}

struct Snake {
    parts: LinkedList<Part>,
    direction: Direction
}

impl Snake {
    fn new() -> Self {
        let mut parts: LinkedList<Part> = LinkedList::new();
        let root_part = Part {x: 500, y: 500};
        parts.push_back(root_part);

        Self {
            parts: parts,
            direction: Direction::Left
        }
    }

    fn draw(&mut self,ctx: &mut Context) {
        for i  in &self.parts {
            let part_rect = graphics::Rect::new_i32(i.x, i.y, 20, 20);
            let part = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), part_rect, Color::WHITE).unwrap();


            if let Err(t) = graphics::draw(ctx, &part, DrawParam::default()) {
                println!("{:?}", t);
            }
        }
    }

    fn add_part(&mut self)  {
        let head_part: &Part = self.parts.back().unwrap();


        match self.direction {
            Direction::Left => {
                self.parts.push_back(Part {x: head_part.x - 10, y: head_part.y});
            },
            Direction::Right => {
                self.parts.push_back(Part {x: head_part.x + 10, y: head_part.y});
            },
            Direction::Up => {
                self.parts.push_back(Part {x: head_part.x, y: head_part.y - 10});
            },
            Direction::Down => {
                self.parts.push_back(Part {x: head_part.x, y: head_part.y + 10});
            }
        }


    }

    fn move_snake(&mut self) {

        //head of the snake
        let mut head: &mut Part = self.parts.front_mut().unwrap();

        let new_block = match self.direction {
            Direction::Up => {
                Part { x: head.x, y: head.y - 10}
            },
            Direction::Down => {
                Part { x: head.x, y: head.y + 10}
            },
            Direction::Right => {
                Part { x: head.x + 10, y: head.y }
            }
            ,Direction::Left => {
                Part { x: head.x - 10, y: head.y}
            }
        };

        self.parts.push_front(new_block);
        self.parts.pop_back();
        
    }

    fn self_collide(&mut self, score: &mut u32, highscore: &mut u32) {
        let mut index = 0;
        let mut collided: bool = false;
        let head: &Part = self.parts.front().unwrap();

        for i  in &self.parts {
            if index > 0 {
                if head.x == i.x && head.y == i.y {
                    collided = true;
                }
            }
            index += 1;
        }

        if collided {
            if *score > *highscore {
                *highscore = *score;
            }
            
            *score = 0;
            self.parts.split_off(1);
        }
    }
}

impl Apple {

    fn new() -> Self {
        Self {
            color: Color::from_rgb(0,200,0),
            width: 20,
            height: 20,
            pos: (100, 100)
        }
    }

    fn new_op(r: u8, g: u8, b: u8, size: i32) -> Self {
        Self {
            color: Color::from_rgb(r,g,b),
            width: size,
            height: size,
            pos: (100, 100)
        }
    }

    fn random_location(&mut self, ctx: &mut Context) -> (i32, i32) {
        let (window_x, window_y) = graphics::size(ctx);
        let x = fastrand::i32(0..window_x as i32 - 100);
        let y = fastrand::i32(0..window_y as i32 - 100);
        self.pos.0 = x;
        self.pos.1 = y;

        (x, y)
    }

    fn get_location(&self) -> (i32, i32) {
        (self.pos.0, self.pos.1)
    }
}

struct State {
	score: u32,
    high_score: u32,
    snake: Snake,
    apple: Apple,
    eat: Eat,
}

impl State {
    fn new() -> Self {
        Self {
            score: 0,
            snake: Snake::new(),
            apple: Apple::new(),
            eat: Eat::Hungry,
            high_score: 0,
        }
    }

}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let (window_x, window_y) = graphics::size(ctx);


        //check for apple
        if ((self.apple.pos.0 - 30 < self.snake.parts.front().unwrap().x && self.apple.pos.0 + 30 >= self.snake.parts.front().unwrap().x) && (self.apple.pos.1 - 30 < self.snake.parts.front().unwrap().y && self.apple.pos.1 + 30 >= self.snake.parts.front().unwrap().y)) {
            self.snake.add_part();
            self.score += 1;
            self.eat = Eat::Hungry;
        } 


        //x checks
        if(self.snake.parts.front().unwrap().x > window_x as i32) {
            self.snake.parts.front_mut().unwrap().x = 0;
        }else if(self.snake.parts.front().unwrap().x < 0) {
            self.snake.parts.front_mut().unwrap().x = window_x as i32;
        }

        //y checks
        if(self.snake.parts.front().unwrap().y > window_y as i32) {
            self.snake.parts.front_mut().unwrap().y = 0;
        }else if(self.snake.parts.front().unwrap().y < 0) {
            self.snake.parts.front_mut().unwrap().y = window_y as i32;
        }
        
        self.snake.move_snake();
        self.snake.self_collide(&mut self.score, &mut self.high_score);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(0,0,128));

        //make apple
        let apple_data: &mut Apple = &mut self.apple;

        let score =  graphics::Text::new(format!("score: {}", self.score));
        let highscore = graphics::Text::new(format!("highscore: {}", self.high_score));
        let high_rect: mint::Point2<f32> = mint::Point2 {x: 180.0, y: 0.0};


        if let Eat::Hungry = self.eat {
            let (apple_x,  apple_y) = apple_data.random_location(ctx);
            let apple_rect = graphics::Rect::new_i32(apple_x, apple_y, apple_data.width, apple_data.height);
            let apple = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), apple_rect, apple_data.color).unwrap();

            if let Err(t) = graphics::draw(ctx, &apple, DrawParam::default()) {
                println!("{:?}", t);
            }

            self.eat = Eat::Full;
        }else {
            let (apple_x,  apple_y) = apple_data.get_location();
            let apple_rect = graphics::Rect::new_i32(apple_x, apple_y, apple_data.width, apple_data.height);
            let apple = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), apple_rect, apple_data.color).unwrap();

            if let Err(t) = graphics::draw(ctx, &apple, DrawParam::default()) {
                println!("{:?}", t);
            }
        }

        graphics::draw(ctx, &score, DrawParam::default());
        graphics::draw(ctx, &highscore, DrawParam::default().dest(high_rect));

        //snake
        self.snake.draw(ctx);


        graphics::present(ctx)
    }

    fn key_down_event(&mut self,ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        match keycode {
            KeyCode::W => {
                if let Direction::Down = self.snake.direction {
                    self.snake.direction = Direction::Down;
                }else {
                    self.snake.direction = Direction::Up;
                }
            },
            KeyCode::S => {
                if let Direction::Up = self.snake.direction {
                    self.snake.direction = Direction::Up;
                }else {
                    self.snake.direction = Direction::Down;
                }
            },
            KeyCode::A => {
                if let Direction::Right = self.snake.direction {
                    self.snake.direction = Direction::Right;
                }else {
                    self.snake.direction = Direction::Left;
                }
            },
            KeyCode::D => {
                if let Direction::Left = self.snake.direction {
                    self.snake.direction = Direction::Left;
                }else {
                    self.snake.direction = Direction::Right;
                }
            },
            _ => {

            }
        }
    }
}

fn main() {
    let game_state = State::new();
    let (context, eventloop) = ContextBuilder::new("Snake", "Me").build().unwrap();
    graphics::set_window_title(&context, "Snake");

    event::run(context, eventloop, game_state);
}
