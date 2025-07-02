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

struct Animal {
    name: String,
    height: f32,
}

impl Animal {
    fn bark(&self) {
        println!("Barking........");
    }

    fn lifespan(&self) -> f32 {
        return 0.5 * self.height;
    }
}

fn main() {
    let dog: Animal = Animal {
        name: String::from("Dhinchak pooja"),
        height: 10.0,
    };

    println!("{}, {}", dog.height, dog.name);
    dog.bark();
    let lifespan: f32 = dog.lifespan();
    println!("lifespan is {}", lifespan);
}
