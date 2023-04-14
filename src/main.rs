use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

mod vector; 
use vector::Vector;

fn main() {
    //We nee this window object to create a window
    let window = Window::new_centered("Pendulum", (1200, 700)).unwrap();

    let win = MyWindowHandler {
        p: Pendulum::new(600.0, 0.0, 200.0),
        p2: Pendulum::new(600.0, 0.0, 400.0),
        p3: Pendulum::new(600.0, 0.0, 600.0)
    };

    //Run the Loop
    window.run_loop(win);
}

struct MyWindowHandler {
    p: Pendulum,
    p2: Pendulum,
    p3: Pendulum,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        //We need to clear the screen every frame time
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        self.p.update();
        self.p.draw(graphics, Color::MAGENTA);

        self.p2.update();
        self.p2.draw(graphics, Color::YELLOW);

        self.p3.update();
        self.p3.draw(graphics, Color::RED);

        //Draw the frame!!
        helper.request_redraw();
    }
}

#[allow(dead_code)]
struct Pendulum {
    //This vector is the position of the pendulum
    origin: Vector,

    //This vector is the position of the ball
    position: Vector,

    angle: f32,

    angular_velocity: f32,
    angular_accelarion: f32,

    r: f32, //The lenght of the pendulum
    m: f32, //The mass of the ball
    g: f32  //The gravity
}

impl Pendulum {
    fn new(x: f32, y:f32, r:f32 ) -> Pendulum{
        Pendulum { 
            origin: Vector::new(x, y), 
            position: Vector { x: 0.0, y: 0.0 }, 
            angle: 1.0, 
            angular_velocity: 0.0, 
            angular_accelarion: 0.0, 
            r: r, 
            m: 1.0, 
            g: 1.5 
        }
    }

    fn update(&mut self) {
        //We use the pendulum equation to calculate the angular acceleration
        self.angular_accelarion = -1.0 * self.g * self.angle.sin() / self.r;

        //The angular velocity is the angular velocity plus tje angular acceleration
        self.angular_velocity += self.angular_accelarion;

        //The angle is the angle plus the angular velocity
        self.angle += self.angular_velocity;

        //The position is the polar coordinates translated to cartesian coordinates
        self.position.set(self.r * self.angle.sin(), self.r * self.angle.cos());

        //The final postion of the ball in the canvas is the origin of the pendulum plus the postion vector
        self.position.add(&self.origin);

    }

    fn draw(&self, graphics: &mut Graphics2D, color: Color) {
        //We need to drae the line of the pendulum first
        //Them, it takes the start and end position, the width and the color of the line
        graphics.draw_line(
            (self.origin.x, self.origin.y), 
            (self.position.x, self.position.y), 
            3.0, 
            color,
        );

        //We need to draw the ball of the pendulum
        //It takes the position, the radius and the color of the ball
        graphics.draw_circle(
            (self.position.x, self.position.y), 
            30.0, 
            color,
        ); 
    }

}
