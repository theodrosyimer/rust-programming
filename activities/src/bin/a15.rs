// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Standard(f64),
    Backstage(f64, String),
    Vip(f64, String),
}

fn main() {
    let tickets = vec![
        Ticket::Standard(25.90),
        Ticket::Backstage(40.90, "Yetu".to_owned()),
        Ticket::Vip(60.90, "Theo".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Standard(price) => println!("Standard ticket price = {:?}", price),
            Ticket::Backstage(price, holder) => {
                println!(
                    "Backstage ticket price = {:?}, holder's name: {:?}",
                    price, holder
                )
            }
            Ticket::Vip(price, holder) => {
                println!(
                    "Backstage ticket price = {:?}, holder's name: {:?}",
                    price, holder
                )
            }
        }
    }
}

// // my version without data-associated in enum
// // and using a struct and impl with #[derive(Debug)]
// // more longer, verbose
// #[derive(Debug)]
// enum TicketType {
//     Standard,
//     Backstage,
//     Vip,
// }

// #[derive(Debug)]
// struct Ticket {
//     price: f64,
//     kind: TicketType,
//     holder_name: String,
// }

// impl Ticket {
//     fn print(&self) {
//         println!("{:?}", self);
//     }
// }

// fn main() {
//     let tickets = vec![
//         Ticket {
//             price: 25.90,
//             holder_name: "Yetu".to_owned(),
//             kind: TicketType::Backstage,
//         },
//         Ticket {
//             price: 40.90,
//             holder_name: "Theo".to_owned(),
//             kind: TicketType::Vip,
//         },
//         Ticket {
//             price: 60.90,
//             holder_name: "".to_owned(),
//             kind: TicketType::Standard,
//         },
//     ];

//     for ticket in tickets {
//         match &ticket {
//             Ticket {
//                 kind: TicketType::Standard,
//                 price,
//                 ..
//             } => {
//                 println!("type: Standard, price = {:?}$", price);
//                 ticket.print();
//                 println!();
//             }
//             Ticket { .. } => {
//                 println!(
//                     "owner name: {:?}, type: {:?}, price = {:?}$",
//                     ticket.holder_name, ticket.kind, ticket.price
//                 );
//                 ticket.print();
//                 println!();
//             }
//         }
//     }
// }
