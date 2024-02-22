// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter
struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn print_quantity(item: &GroceryItem) {
    println!("Quantity: {:?}", item.quantity)
}

fn print_id(item: &GroceryItem) {
    println!("Id: {:?}", item.id)
}

fn main() {
    let grocery_item = GroceryItem {
        quantity: 31,
        id: 12,
    };
    print_quantity(&grocery_item);
    print_id(&grocery_item);
}
