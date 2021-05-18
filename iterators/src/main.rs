use std::env::{args, Args};
use std::iter::Skip;

// fn main() {
//     let mut args = args();

//     loop {
//         match args.next() {
//             None => break,
//             x => println!("{:?}", x)
//         }
//     }
// }

// Nicer

// fn main() {
//     let mut args = args();

//     while let Some(arg) = args.next() {
//         println!("{}", arg);
//     }
// }

// Even Nicer

// fn main() {
//     for arg in args().skip(1) {
//         println!("{}", arg);
//     }
// }

// Challenge

// fn main() {
//     let args: Skip<Args> = args().skip(1);
//     for arg in args {
//         println!("{}", arg);
//     }
// }

// Parsing Integers

// fn main() {
//     for arg in args().skip(1) {
//         println!("{:?}", arg.parse::<u32>());
//     }
// }