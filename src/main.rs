extern crate coffee;

use coffee::graphics::{
    Color, Frame, Mesh, Rectangle, Shape, Window, WindowSettings,
};
use coffee::input::{self, keyboard, Input};
use coffee::input::keyboard::KeyCode;

use coffee::load::Task;
use coffee::{Game, Timer};

fn main() -> coffee::Result<()> {
    SnakeGame::run(WindowSettings {
        title: String::from("Snake"),
        size: (900, 600),
        resizable: true,
        maximized: false,
        fullscreen: false,
    })
}

#[derive(Debug,Clone,Copy)]
struct Position(f32, f32);

struct Snake {
	square_pos: Vec<Position>,
}

impl Snake{

	fn new() -> Snake {
		Snake {
			square_pos: vec![],
		}
	}
	fn create_snake(&mut self){
		for i in 1..4{
			let x = (30.0*i as f32) + 30.0;
			let y = 90.0;
			self.square_pos.push(Position(x, y));
		}
	}
	fn draw_snake(&mut self, frame: &mut Frame){
		let mut mesh = Mesh::new();
		for pos in &self.square_pos{
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

	fn move_right(&mut self){
		//let new_x = Position(self.square_pos[0].0 + 30.0, self.square_pos[0].1);
		/*println!("a");
		self.square_pos.remove(0);*/
		println!("{:?}",self.square_pos);
		let head = self.square_pos.last_mut().unwrap();
		head.0 += 30.0;
	}
}

struct SnakeGame{
	snake: Snake
}

impl Game for SnakeGame {

    type Input = CustomInput;
    type LoadingScreen = ();

    
    fn load(_window: &Window) -> Task<SnakeGame> {
        Task::succeed(|| SnakeGame{ 
        	snake : Snake::new()
        })
    }

    fn interact(&mut self, input: &mut CustomInput, _window: &mut Window) {
    	match input.keys_pressed.last(){
    		Some(KeyCode::Right) => {
    			self.snake.move_right();
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