#[derive(Debug)]
struct Person {
    name: String,
    age: u32
}

fn birthday_immutable(person: Person) -> Person {
    Person {
        name: person.name,
        age: person.age + 1
    }
}

fn birthday_mutable(mut person: Person) -> Person {
    person.age += 1;
    person
}

pub fn main() {
    let alice1 = Person { name: String::from("Alice"), age: 30 };
    println!("Alice 1: {:?}", alice1);
    let alice2 = birthday_immutable(alice1);
    println!("Alice 2: {:?}", alice2);
    let alice3 = birthday_mutable(alice2);
    println!("Alice 3: {:?}", alice3);
}
