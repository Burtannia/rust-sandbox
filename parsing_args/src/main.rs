use std::env::{args, Args};
use std::iter::Skip;

#[derive(Debug)]
struct Frame {
    width: u32,
    height: u32
}

#[derive(Debug)]
enum ParseError {
    TooManyArgs,
    TooFewArgs,
    InvalidInteger(String)
}

// This is an abomination...
// fn parse_args() -> Result<Frame, ParseError> {
//     use self::ParseError::*;

//     let mut args = std::env::args().skip(1);

//     match args.next() {
//         None => Err(TooFewArgs),
//         Some(width_str) => {
//             match args.next() {
//                 None => Err(TooFewArgs),
//                 Some(height_str) => {
//                     match args.next() {
//                         Some(_) => Err(TooManyArgs),
//                         None => {
//                             match width_str.parse() {
//                                 Err(_) => Err(InvalidInteger(width_str)),
//                                 Ok(width) => {
//                                     match height_str.parse() {
//                                         Err(_) => Err(InvalidInteger(height_str)),
//                                         Ok(height) => Ok(Frame {
//                                             width,
//                                             height
//                                         })
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

// Still suboptimal
// fn parse_args() -> Result<Frame, ParseError> {
//     use self::ParseError::*;
//     let mut args = std::env::args().skip(1);

//     let width_str = match args.next() {
//         None => return Err(TooFewArgs),
//         Some(width_str) => width_str,
//     };
    
//     let height_str = match args.next() {
//         None => return Err(TooFewArgs),
//         Some(height_str) => height_str,
//     };

//     match args.next() {
//         Some(_) => return Err(TooManyArgs),
//         None => (),
//     }

//     let width = match width_str.parse() {
//         Err(_) => return Err(InvalidInteger(width_str)),
//         Ok(width) => width,
//     };

//     let height = match height_str.parse() {
//         Err(_) => return Err(InvalidInteger(height_str)),
//         Ok(height) => height,
//     };

//     Ok(Frame {
//         width,
//         height,
//     })
// }

fn parse_u32(s: String) -> Result<u32, ParseError> {
    match s.parse() {
        Err(_) => Err(ParseError::InvalidInteger(s)),
        Ok(x) => Ok(x)
    }
}

struct ParseArgs(std::env::Args);

impl ParseArgs {
    fn new() -> ParseArgs {
        ParseArgs(std::env::args())
    }

    fn require_arg(&mut self) -> Result<String, ParseError> {
        match self.0.next() {
            None => Err(ParseError::TooFewArgs),
            Some(x) => Ok(x)
        }
    }

    fn require_no_args(&mut self) -> Result<(), ParseError> {
        match self.0.next() {
            Some(_) => Err(ParseError::TooManyArgs),
            None => Ok(())
        }
    }
}

fn parse_args() -> Result<Frame, ParseError> {
    let mut args = ParseArgs::new();

    // skip command name
    args.require_arg()?;

    let width_str = args.require_arg()?;
    let height_str = args.require_arg()?;

    args.require_no_args()?;

    let width = parse_u32(width_str)?;
    let height = parse_u32(height_str)?;

    Ok(Frame { width, height } )
}

fn main() {
    println!("{:?}", parse_args())
}