// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut store = HashMap::new();
    store.insert("chair", 5);
    store.insert("bed", 3);
    store.insert("table", 2);
    store.insert("couch", 0);

    let mut total_count: i32 = 0;

    for (item, qty) in store.iter() {
        total_count = total_count + qty;

        // need to borrow the integer 0
        // because key-value pairs in HashMap
        // are automatically borrowed
        let stock_count = if qty == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", qty)
        };
        println!("item: {:?}, stock = {:?}", item, stock_count);
    }
    println!("total stock = {:?}", total_count);
}
