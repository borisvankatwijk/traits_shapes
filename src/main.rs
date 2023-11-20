use std::io::Error;

#[derive(Debug)]
enum Shape {
    // Two dimensional
    Square { side: f32 },
    Rectangle { length: f32, width: f32 },
    Circle { radius: f32 },
    // Three dimensional
    Cube { width: f32, height: f32, depth: f32 },
    Sphere { radius: f32 },
}

impl Shape {
    fn new_square() -> Shape {
        Shape::Square { side: 0.0 }
    }

    fn new_rectangle() -> Shape {
        Shape::Rectangle { length: 0.0, width: 0.0 }
    }

    fn new_circle() -> Shape {
        Shape::Circle { radius: 0.0 }
    }

    fn new_cube() -> Shape {
        Shape::Cube { width: 0.0, height: 0.0, depth: 0.0 }
    }

    fn new_sphere() -> Shape {
        Shape::Sphere { radius: 0.0 }
    }
}

impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Square { side } => side * side,
            Shape::Rectangle { length, width } => length * width,
            Shape::Circle { radius } => std::f32::consts::PI * radius * radius,
            Shape::Cube { width, height, depth } => {
                // Add all area's of the cube
                2.0 * width * height // two sides
                    + 2.0 * width * depth // next two sides
                    + 2.0 * height * depth // final two sides
            }
            Shape::Sphere { radius } => 4.0 * std::f32::consts::PI * radius * radius,
        }
    }

    fn volume(&self) -> f32 {
        match self {
            Shape::Cube { width, height, depth } => width * height * depth,
            Shape::Sphere { radius } => 4.0 / 3.0 * std::f32::consts::PI * radius * radius * radius,
            _ => panic!("Not a 3D shape"),
        }
    }

    fn print_aspects(&self) -> () {
        match self {
            Shape::Square { side } => {
                println!(
                    "Aspects of square: s={}m, area={}m2",
                    side,
                    self.area()
                );
            }
            Shape::Rectangle { length, width } => {
                println!(
                    "Aspects of rectangle: l={}m, w={}m, area={}m2",
                    length,
                    width,
                    self.area()
                );
            }
            Shape::Circle { radius } => {
                println!(
                    "Aspects of circle: r={}m, area={}m2",
                    radius,
                    self.area()
                );
            }
            Shape::Cube { width, height, depth } => {
                println!(
                    "Aspects of cube: w={}m, h={}m, d={}m, area={}m2, volume={}m3",
                    width,
                    height,
                    depth,
                    self.area(),
                    self.volume()
                );
            }
            Shape::Sphere { radius } => {
                println!(
                    "Aspects of sphere: r={}m, area={}m2, volume={}m3",
                    radius,
                    self.area(),
                    self.volume()
                );
            }
        }
    }
}

// pub fn area<T: TwoDimensional> (shape: &T) -> f32 {
//     shape.area()
// }


trait HasRadius {
    fn increase_radius(&mut self, increase: f32) -> Result<(), Error>;
    fn decrease_radius(&mut self, decrease: f32) -> Result<(), Error>;
}

impl HasRadius for Shape {
    fn increase_radius(&mut self, increase: f32) -> Result<(), Error> {
        // See if the Shape type we have has attribute radius
        match self {
            Shape::Circle { radius } => {
                *radius += increase;
                Ok(())
            }
            Shape::Sphere { radius } => {
                *radius += increase;
                Ok(())
            }
            _ => Err(Error::new(std::io::ErrorKind::Other, "Not a shape with radius"))
        }
    }

    fn decrease_radius(&mut self, decrease: f32) -> Result<(), Error> {
        match self {
            Shape::Circle { radius } => {
                *radius = *radius - decrease;
                if *radius < 0.0 {
                    println!(
                        "\x1b[31mNew radius would become {}, so we changed it to 0.0\x1b[0m",
                        *radius
                    );
                    *radius = 0.0
                }
                Ok(())
            }
            Shape::Sphere { radius } => {
                *radius = *radius - decrease;
                if *radius < 0.0 {
                    println!(
                        "\x1b[31mNew radius would become {}, so we changed it to 0.0\x1b[0m",
                        *radius
                    );
                    *radius = 0.0
                }
                Ok(())
            }
            _ => Err(Error::new(std::io::ErrorKind::Other, "Not a shape with radius"))
        }
    }
}

fn main() {
    println!("== Play around with traits by using shapes");

    let square = Shape::new_square();
    let mut rectangle = Shape::new_rectangle();
    let circle = Shape::new_circle();
    let cube = Shape::new_cube();
    let mut sphere = Shape::new_sphere();

    println!("Area of square: {}", square.area());
    println!("Area of rectangle: {}", rectangle.area());
    println!("Area of circle: {}", circle.area());
    println!("Volume of cube: {}", cube.volume());
    println!("Volume of sphere: {}", sphere.volume());

    sphere.print_aspects();
    sphere.increase_radius(2.0 * std::f32::consts::E).expect("Increase of radius");
    sphere.print_aspects();
    sphere.decrease_radius(std::f32::consts::LN_2).expect("Increase of radius by ln(2)");
    sphere.decrease_radius(500.0).expect("Decrease of radius by 500.0 (resulting in 0.0)");
    sphere.print_aspects();

    // Should be empty
    assert_eq!(sphere.volume(), 0.0);

    let error_string = rectangle.increase_radius(2.0)
        .expect_err("Should have an error");
    println!("Error: {}", error_string);
}
