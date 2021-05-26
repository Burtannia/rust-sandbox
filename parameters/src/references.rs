#[derive(Debug)]
struct Person {
    name: String,
    age: u32
}

fn birthday_immutable(person: &mut Person) {
    person.age += 1;
}

fn birthday_mutable<'a>(mut person: &'a mut Person, replacement: &'a mut Person) {
    person = replacement;
    person.age += 1;
}

pub fn main() {
    let mut alice = Person { name: String::from("Alice"), age: 30 };
    let mut bob = Person { name: String::from("Bob"), age: 20 };
    println!("Alice 1: {:?}, Bob 1: {:?}", alice, bob);
    birthday_immutable(&mut alice);
    println!("Alice 2: {:?}, Bob 2: {:?}", alice, bob);
    birthday_mutable(&mut alice, &mut bob);
    println!("Alice 3: {:?}, Bob 3: {:?}", alice, bob); 
}

// does not compile because the person binding is immutable therefore person = replacement fails
// fn birthday_immutable_broken<'a>(person: &'a mut Person, replacement: &'a mut Person)
// {
//     person = replacement;
//     person.age += 1;
// }

// In summary:
// If the reference is mutable we can change the value.
// If the variable the reference is bound to is mutable then we can change what that variable points to.
// If a function requires a mutable reference then the variable in the caller must be mutable.
// A mutable reference can also be passed to anything where an immutable reference is required.