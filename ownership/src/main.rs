#[derive(Debug, Clone)]
struct Foobar(i32);

impl Drop for Foobar {
    fn drop(&mut self) {
        println!("Dropping a Foobar: {:?}", self);
    }
}

// fn uses_foobar(foobar: &Foobar) {
//     println!("I consumed a Foobar: {:?}", foobar);
// }

////////////////////

// fn main() {
//     println!("Before x");
//     let _x = Foobar(1);
//     println!("After x");
//     {
//         println!("Before y");
//         let _y = Foobar(2);
//         println!("After y");
//     }
//     println!("End of main");
// }

// Borrowing (Immutable)

// fn main() {
//     let x = Foobar(1);
//     println!("Before uses_foobar");
//     uses_foobar(&x);
//     uses_foobar(&x);
//     println!("After uses_foobar");
// }

// Object Syntax

// impl Foobar {
//     fn use_it(&self) {
//         println!("I consumed a Foobar: {:?}", self);
//     }
// }

// fn main() {
//     let x = Foobar(1);
//     println!("Before uses_foobar");
//     x.use_it();
//     x.use_it();
//     println!("After uses_foobar");
// }

/////////////////////////
// Multiple References //
/////////////////////////

// fn main() {
//     //mult_good();
//     //mult_err();
//     mult_mut();
// }

// fn mult_good() {
//     let x: Foobar = Foobar(1);
//     let y: &Foobar = &x;
//     println!("Before uses_foobar");
//     uses_foobar(&x);
//     uses_foobar(y);
//     println!("After uses_foobar");
// }

// fn mult_err() {
//     let x: Foobar = Foobar(1);
//     let y: &Foobar = &x;
//     println!("Before uses_foobar");
//     uses_foobar(&x);
//     std::mem::drop(x);
//     uses_foobar(y);
//     println!("After uses_foobar");
// }

// fn mult_mut() {
//     let mut x: Foobar = Foobar(1);
//     let y: &mut Foobar = &mut x;
//     println!("Before uses_foobar");
//     uses_foobar(&x); // will fail!
//     uses_foobar(y);
//     println!("After uses_foobar");
// }

// Bad Stuff

// fn main() {
//     let x = Foobar(1);

//     foo(x);
//     foo(x); // should error because x has already moved into foo

//     let mut y = Foobar(2);

//     bar(&y); 
//     bar(&y);

//     let z = &mut y; // mutable borrow

//     bar(&y); // immutable borrow not allowed because y already mutably borrowed
//     baz(&mut y); // second mutable borrow not allowed
//     baz(z);
// }

// // move
// fn foo(_foobar: Foobar) {
// }

// // read only reference
// fn bar(_foobar: &Foobar) {
// }

// // mutable reference
// fn baz(_foobar: &mut Foobar) {
// }

// Mutating an "immutable" variable
// this mutation is allowed because the immutability applies only to variable x
// not to the value we are passing, once x is moved to foo, main doesn't care.

// fn main() {
//     let x = Foobar(1);
//     foo(x);
// }

// fn foo(mut x: Foobar) {
//     x.0 = 2; // changes the 0th value inside the product
//     println!("{:?}", x);
// }

// Copying & Cloning

// fn uses_foobar(foobar: Foobar) {
//     println!("I consumed a Foobar: {:?}", foobar);
// }

// fn main() {
//     let x = Foobar(1);
//     uses_foobar(x.clone());
//     uses_foobar(x);
// }

// #[derive(Debug, Clone, Copy)]
// struct Foobar(i32);

// fn uses_foobar(foobar: Foobar) {
//     println!("I consumed a Foobar: {:?}", foobar);
// }

// fn main() {
//   let x = Foobar(1);
//   uses_foobar(x);
//   uses_foobar(x);
// }
