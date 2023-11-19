trait TwoDimensional {
    fn area(&self) -> f32;
}

trait ThreeDimensional {
    fn volume(&self) -> f32;
}

struct Square {
    side: f32,
}

struct Rectangle {
    length: f32,
    width: f32,
}

struct Circle {
    radius: f32,
}

struct Cube {
    length: f32,
    width: f32,
    height: f32,
}

struct Sphere {
    radius: f32,
}

impl TwoDimensional for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }
}

impl TwoDimensional for Rectangle {
    fn area(&self) -> f32 {
        self.length * self.width
    }
}

impl TwoDimensional for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}

impl ThreeDimensional for Cube {
    fn volume(&self) -> f32 {
        self.length * self.width * self.height
    }
}

impl ThreeDimensional for Sphere {
    fn volume(&self) -> f32 {
        (4.0 / 3.0) * std::f32::consts::PI * self.radius * self.radius * self.radius
    }
}

impl TwoDimensional for Sphere {
    fn area(&self) -> f32 {
        4.0 * std::f32::consts::PI * self.radius * self.radius
    }
}

impl Sphere {
    fn increase_radius(&mut self, increase: f32) {
        self.radius += increase;
    }

    fn decrease_radius(&mut self, decrease: f32) {
        self.radius = self.radius - decrease;
        if self.radius < 0.0 {
            println!("\x1b[31mNew radius would become {}, so we changed it to 0.0\x1b[0m", self.radius);
            self.radius = 0.0
        }
    }

    fn print_aspects(&self) -> () {
        println!("Aspects of sphere: r={}m, area={}m2, volume={}m3", self.radius, self.area(), self.volume());
    }
}

fn main() {
    println!("== Play around with traits by using shapes");

    let square = Square { side: 5.0 };
    let rectangle = Rectangle { length: 5.0, width: 14.0 };
    let circle = Circle { radius: 3.0 };
    let cube = Cube { length: 2.0, width: 2.0, height: 2.0 };
    let mut sphere = Sphere { radius: std::f32::consts::E };

    println!("Area of square: {}", square.area());
    println!("Area of rectangle: {}", rectangle.area());
    println!("Area of circle: {}", circle.area());
    println!("Volume of cube: {}", cube.volume());
    println!("Volume of sphere: {}", sphere.volume());

    sphere.print_aspects();
    sphere.increase_radius(2.0 * std::f32::consts::E);
    sphere.print_aspects();
    sphere.decrease_radius(std::f32::consts::LN_2);
    sphere.decrease_radius(500.0);
    sphere.print_aspects();
}
