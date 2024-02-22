// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Orange,
    Banana,
    Peach,
}

struct Drink {
    flavor: Flavor,
    ounces: f64,
}

fn print_drink_infos(drink: Drink) {
    match drink.flavor {
        Flavor::Banana => println!("flavor: banana"),
        Flavor::Orange => println!("flavor: orange"),
        Flavor::Peach => println!("flavor: peach"),
    };
    println!("weight in ounces: {:?}", drink.ounces)
}

fn main() {
    let my_drink = Drink {
        flavor: Flavor::Banana,
        ounces: 1.2,
    };

    print_drink_infos(my_drink);
}
