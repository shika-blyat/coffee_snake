extern crate coffee;
use coffee::graphics::{
    Color, Frame, Mesh, Rectangle, Shape, Window, WindowSettings,
};
use coffee::load::Task;
use coffee::{Game, Timer};

fn main() -> coffee::Result<()> {
    Example::run(WindowSettings {
        title: String::from("A little snake game"),
        size: (800, 600),
        resizable: true,
        maximized: false,
        fullscreen: false,
    })
}

struct Example;

impl Example{
	fn draw_snake(&mut self, frame: &mut Frame){
		let mut mesh = Mesh::new();
        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: 50.0,
                y: 50.0,
                width: 50.0,
                height: 50.0,
            }),
            Color::WHITE,
        );
        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: 50.0,
                y: 100.0,
                width: 50.0,
                height: 50.0,
            }),
            Color::BLACK,
        );
        mesh.draw(&mut frame.as_target());
	}
}

impl Game for Example {

    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Example> {
        Task::succeed(|| Example)
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::new(0.5, 0.5, 0.5, 1.0));
        self.draw_snake(frame);        
    }
}