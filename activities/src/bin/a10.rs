// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_message(my_bool: bool) {
    match my_bool {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn main() {
    let n = 99;
    let is_gt_100 = n > 100;

    print_message(is_gt_100);
}
