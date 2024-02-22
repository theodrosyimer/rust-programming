// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum Color {
    None,
    Green,
    Yellow,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::None => println!("Color: none"),
            Color::Green => println!("Color: green"),
            Color::Yellow => println!("Color: yellow"),
            Color::Red => println!("Color: red"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
        println!("Depth: {:?}", self.depth);
    }
}
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }
    fn print(&self) {
        println!("Weight: {:?}", self.weight);
        self.color.print();
        self.dimensions.print();
    }
}

fn main() {
    let small_dimension = Dimensions {
        width: 2.5,
        height: 2.0,
        depth: 2.0,
    };
    let my_box = ShippingBox::new(2.5, Color::Green, small_dimension);
    my_box.print();
}
