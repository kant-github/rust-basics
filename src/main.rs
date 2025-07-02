// fn main() {
//     let string: String = String::from("Rishi Kant");
//     let size: usize = get_length(&string);
//     println!("{}", size);
//     println!("{}", string);
// }

// fn get_length(str: &String) -> usize {
//     return str.len();
// }
// --------------------------------------------------------------------------------->

//

// --------------------------------------------------------------------------------->

// enum Direction {
//     North,
//     South,
//     East,
//     West,
// }

// fn main() {
//     let movement: Direction = Direction::South;
//     steer(movement);
// }

// fn steer(direction: Direction) {
//     match direction {
//         Direction::East => println!("Moved east"),
//         Direction::West => println!("Moved west"),
//         Direction::North => println!("Moved north"),
//         _ => println!("Moved dont know"),
//     }
// }

// --------------------------------------------------------------------------------->

enum Shapes {
    Circle(f32),
    Rectangle(f32, f32),
    Square(f32),
}

fn main() {
    let shape_circle: Shapes = Shapes::Circle(10.0);
    calculate_area(shape_circle);
    let shape_rect: Shapes = Shapes::Rectangle(10.0, 10.0);
    calculate_area(shape_rect);
    let shape_square: Shapes = Shapes::Square(20.0);
    calculate_area(shape_square);
}

fn calculate_area(shape: Shapes) {
    match shape {
        Shapes::Circle(radius) => {
            let area: f32 = std::f32::consts::PI * radius * radius;
            println!("Area of circle is : {}", area);
        }
        Shapes::Rectangle(width, height) => {
            let area: f32 = width * height;
            println!("Area of rectangle is : {}", area);
        }
        Shapes::Square(side) => {
            let area: f32 = side * side;
            println!("Area of swuare is : {}", area);
        }
    }
}
