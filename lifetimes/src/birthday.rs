#[derive(Debug)]
struct Person {
    name: Option<String>,
    age: Option<u32>,
}

fn print_person(mut person: Person) {

    match person.name {
        Some(ref name) => println!("Name is {}", name),
        None => println!("No name provided"),
    }

    match person.age {
        Some(ref mut age) => {
            println!("Age is {}", age);
            *age += 1;
        }
        None => println!("No age provided"),
    }

    println!("Full Person value: {:?}", person);
}

pub fn main() {
    print_person(Person {
        name: Some(String::from("Alice")),
        age: Some(30),
    });
}