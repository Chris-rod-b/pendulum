mod vector; 
use vector::Vector;

fn main() {
    println!("Hello, world!");
}

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

    fn draw() {

    }
}