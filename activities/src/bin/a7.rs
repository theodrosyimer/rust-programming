// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Black,
    White,
    Green,
    Blue,
}

fn print_color_name(color: Color) {
    match color {
        Color::Black => println!("black"),
        Color::White => println!("white"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
    }
}

fn main() {
    print_color_name(Color::Black);
}
