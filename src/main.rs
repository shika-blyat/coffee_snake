extern crate coffee;

use coffee::graphics::{Color, Frame, Mesh, Rectangle, Shape, Window, WindowSettings};
use coffee::input::keyboard::KeyCode;
use coffee::input::{self, keyboard, Input};

use coffee::load::Task;
use coffee::{Game, Timer};

use rand::seq::IteratorRandom;

fn main() -> coffee::Result<()> {
    SnakeGame::run(WindowSettings {
        title: String::from("Snake"),
        size: (900, 600),
        resizable: false,
        maximized: false,
        fullscreen: false,
    })
}

fn new_random_pos() -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let x = (0..900).step_by(30).choose(&mut rng).unwrap() as f32;
    let y = (0..600).step_by(30).choose(&mut rng).unwrap() as f32;
    return (x,y)
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Position(f32, f32);

struct Snake {
    square_pos: Vec<Position>,
    apple: Apple,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            square_pos: vec![],
            apple: Apple::new(),
        }
    }
    fn create_snake(&mut self) {
        for i in 1..6 {
            let x = (30.0 * i as f32) + 30.0;
            let y = 90.0;
            self.square_pos.push(Position(x, y));
        }
    }
    fn draw_snake_and_apple(&mut self, mut frame: &mut Frame) {
        let mut mesh = Mesh::new();
        for pos in &self.square_pos {
            mesh.fill(
                Shape::Rectangle(Rectangle {
                    x: pos.0,
                    y: pos.1,
                    width: 30.0,
                    height: 30.0,
                }),
                Color::RED,
            );
        }
        mesh.draw(&mut frame.as_target());
        self.apple.draw(&mut frame);
    }

    fn move_right(&mut self) {
        self.square_pos.remove(0);
        let head = self.square_pos.last().unwrap().clone();
        if head.0 != 870.0 {
            self.square_pos.push(Position(head.0 + 30.0, head.1));
        } else {
            self.square_pos.push(Position(0.0, head.1));
        }
    }
    fn move_left(&mut self) {
        self.square_pos.remove(0);
        let head = self.square_pos.last().unwrap().clone();
        if head.0 != 0.0 {
            self.square_pos.push(Position(head.0 - 30.0, head.1));
        } else {
            self.square_pos.push(Position(900.0, head.1));
        }
    }
    fn move_bottom(&mut self) {
        self.square_pos.remove(0);
        let head = self.square_pos.last().unwrap().clone();
        if head.1 != 570.0 {
            self.square_pos.push(Position(head.0, head.1 + 30.0));
        } else {
            self.square_pos.push(Position(head.0, 0.0));
        }
    }
    fn move_top(&mut self) {
        self.square_pos.remove(0);
        let head = self.square_pos.last().unwrap().clone();
        if head.1 != 0.0 {
            self.square_pos.push(Position(head.0, head.1 - 30.0));
        } else {
            self.square_pos.push(Position(head.0, 600.0));
        }
    }
    fn move_to(&mut self, keycode: Option<KeyCode>) {
        match keycode {
            Some(KeyCode::Right) => {
                self.move_right();
            }
            Some(KeyCode::Left) => {
                self.move_left();
            }
            Some(KeyCode::Down) => {
                self.move_bottom();
            }
            Some(KeyCode::Up) => {
                self.move_top();
            }
            _ => (),
        }
    }
    fn ate_apple(&self) -> bool{
        if self.apple.pos == *self.square_pos.last().unwrap(){
            return true
        }
        false
    }
}

struct Apple {
    pub pos: Position,
    eaten: bool
}

impl Apple {
    fn new() -> Apple {
        let (x, y) = new_random_pos();
        Apple {
            pos: Position(x, y),
            eaten: false
        }
    }
    fn draw(&mut self, frame: &mut Frame) {
        let mut mesh = Mesh::new();
        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: self.get_pos().0,
                y: self.get_pos().1,
                width: 30.0,
                height: 30.0,
            }),
            Color::GREEN,
        );
        mesh.draw(&mut frame.as_target());
    }
    fn get_pos(&mut self) -> Position {
        if self.eaten {
            let (x,y) = new_random_pos();
            self.pos = Position(x,y);
            self.eaten = false;
        }
        self.pos
    }
}
struct SnakeGame {
    snake: Snake,
    last_key: Option<KeyCode>,
    score: u16,
}

impl Game for SnakeGame {
    const TICKS_PER_SECOND: u16 = 3;
    type Input = CustomInput;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<SnakeGame> {
        let mut snake = Snake::new();        
        snake.create_snake();
        Task::succeed(|| SnakeGame {
            snake,
            score: 0,
            last_key: None, 
        })        
    }
    fn update(&mut self, _window: &Window) {
        self.snake.move_to(self.last_key);
    }
    fn interact(&mut self, input: &mut CustomInput, _window: &mut Window) {
        let key = input.keys_pressed.last();
        match key {
            Some(KeyCode::Right) => {
                if Some(KeyCode::Left) != self.last_key {
                    self.last_key = Some(*key.unwrap());
                }
            }
            Some(KeyCode::Left) => {
                if Some(KeyCode::Right) != self.last_key {
                    self.last_key = Some(*key.unwrap());
                }
            }
            Some(KeyCode::Down) => {
                if Some(KeyCode::Up) != self.last_key {
                    self.last_key = Some(*key.unwrap());
                }
            }
            Some(KeyCode::Up) => {
                if Some(KeyCode::Down) != self.last_key {
                    self.last_key = Some(*key.unwrap());
                }
            }

            None | _ => (),
        }
    }
    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        if self.snake.ate_apple(){
            self.snake.apple.eaten = true;
            self.score += 1;
            println!("current score: {}", self.score);
        }
        frame.clear(Color::new(0.5, 0.5, 0.5, 1.0));
        self.snake.draw_snake_and_apple(frame);
    }
}

struct CustomInput {
    keys_pressed: Vec<KeyCode>,
}

impl Input for CustomInput {
    fn new() -> CustomInput {
        CustomInput {
            keys_pressed: vec![],
        }
    }

    fn update(&mut self, event: input::Event) {
        match event {
            input::Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::Input { key_code, state } => match state {
                    input::ButtonState::Pressed => {
                        self.keys_pressed.push(key_code);
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }

    fn clear(&mut self) {
        self.keys_pressed.clear();
    }
}