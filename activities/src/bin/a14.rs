// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}

fn print(data: &str) {
    println!("{:?}", data)
}

fn main() {
    let people = vec![
        Person {
            age: 6,
            name: String::from("Yetu"),
            favorite_color: String::from("purple"),
        },
        Person {
            age: 7,
            name: String::from("Antoine"),
            favorite_color: String::from("red"),
        },
        Person {
            age: 11,
            name: String::from("Theo"),
            favorite_color: String::from("orange"),
        },
    ];
    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.favorite_color);
        }
    }
}
