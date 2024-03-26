extern crate piston_window;

use piston_window::*;

struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    rotation: f64,
    color: [f32; 4],
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64, rotation: f64, color: [f32; 4]) -> Rect {
        Rect {
            x,
            y,
            width,
            rotation,
            height,
            color,
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        let c = c.rot_deg(self.rotation);
        rectangle(
            self.color,
            [self.x, self.y, self.width, self.height],
            c.transform,
            g,
        );
    }
}

struct App {
    window: PistonWindow,
    rect_collection: Vec<Rect>,
}

impl App {
    pub fn new(size: [u32; 2]) -> App {
        let opengl = OpenGL::V3_2;
        let mut window: PistonWindow = WindowSettings::new("shapes", size)
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();
        window.set_lazy(true);

        let mut rect_collection: Vec<Rect> = vec![];
        rect_collection.push(Rect::new(20.0, 20.0, 60.0, 60.0, 0.0, [1.0, 0.0, 0.0, 1.0]));
        rect_collection.push(Rect::new(80.0, 20.0, 60.0, 60.0, 0.0, [0.0, 1.0, 0.0, 1.0]));
        rect_collection.push(Rect::new(20.0, 80.0, 60.0, 60.0, 0.0, [0.0, 0.0, 1.0, 1.0]));

        App {
            window,
            rect_collection,
        }
    }

    pub fn run(&mut self) {
        while let Some(e) = self.window.next() {
            self.window.draw_2d(&e, |c, g, _| {
                clear([1.0; 4], g);
                for rect in &self.rect_collection {
                    rect.render(c, g);
                }
            });
        }
    }
}

fn main() {
    let mut app = App::new([800, 600]);

    app.run();
}
