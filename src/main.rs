extern crate coffee;

use coffee::graphics::{Color, Frame, Mesh, Rectangle, Shape, Window, WindowSettings};
use coffee::input::keyboard::KeyCode;
use coffee::input::{self, keyboard, Input};

use coffee::load::Task;
use coffee::{Game, Timer};

fn main() -> coffee::Result<()> {
    SnakeGame::run(WindowSettings {
        title: String::from("Snake"),
        size: (900, 600),
        resizable: false,
        maximized: false,
        fullscreen: false,
    })
}

#[derive(Debug, Clone, Copy)]
struct Position(f32, f32);

struct Snake {
    square_pos: Vec<Position>,
}

impl Snake {
    fn new() -> Snake {
        Snake { square_pos: vec![] }
    }
    fn create_snake(&mut self) {
        for i in 1..6 {
            let x = (30.0 * i as f32) + 30.0;
            let y = 90.0;
            self.square_pos.push(Position(x, y));
        }
    }
    fn draw_snake(&mut self, frame: &mut Frame) {
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
    }

    fn move_right(&mut self) {
        self.square_pos.remove(0);
        let head = self.square_pos.last().unwrap().clone();
        if head.0 != 870.0{
        	self.square_pos.push(Position(head.0 + 30.0, head.1));
    	} else {
    		self.square_pos.push(Position(0.0, head.1));
    	}
    }
    fn move_left(&mut self) {
        self.square_pos.remove(0);
        let head = self.square_pos.last().unwrap().clone();
        if head.0 != 0.0{
        	self.square_pos.push(Position(head.0 - 30.0, head.1));
    	} else {
    		self.square_pos.push(Position(900.0, head.1));
    	}         
    }
    fn move_bottom(&mut self) {
        self.square_pos.remove(0);
        let head = self.square_pos.last().unwrap().clone();
        if head.1 != 570.0{
        	self.square_pos.push(Position(head.0, head.1 + 30.0));
    	} else {
    		self.square_pos.push(Position(head.0, 0.0));
    	}        
    }
    fn move_top(&mut self) {
        self.square_pos.remove(0);
        let head = self.square_pos.last().unwrap().clone();
        if head.1 != 0.0{
        	self.square_pos.push(Position(head.0, head.1 - 30.0));
    	} else {
    		self.square_pos.push(Position(head.0, 600.0));
    	} 
        
    }
    fn move_to(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Right => {
                if keycode != KeyCode::Left {
                    self.move_right();
                }
            }
            KeyCode::Left => {
                self.move_left();
            }
            KeyCode::Down => {
                self.move_bottom();
            }
            KeyCode::Up => {
                self.move_top();
            }
            _ => (),
        }
    }
}

struct SnakeGame {
    snake: Snake,
    last_key: KeyCode,
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
            last_key: KeyCode::F24, // Here F24 is only used as a default value.
        })
    }
    fn update(&mut self, _window: &Window) {
        self.snake.move_to(self.last_key);
    }
    fn interact(&mut self, input: &mut CustomInput, _window: &mut Window) {
        let key = input.keys_pressed.last();
        match key {
            Some(KeyCode::Right) => {
                if KeyCode::Left != self.last_key {
                    self.last_key = *key.unwrap();
                }
            }
            Some(KeyCode::Left) => {
                if KeyCode::Right != self.last_key {
                    self.last_key = *key.unwrap();
                }
            }
            Some(KeyCode::Down) => {
                if KeyCode::Up != self.last_key {
                    self.last_key = *key.unwrap();
                }
            }
            Some(KeyCode::Up) => {
                if KeyCode::Down != self.last_key {
                    self.last_key = *key.unwrap();
                }
            }

            None | _ => (),
        }
    }
    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::new(0.5, 0.5, 0.5, 1.0));
        self.snake.draw_snake(frame);
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
