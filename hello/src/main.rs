fn main() {
    let james = Person {
        name: String::from("James"),
        age: 24
    };
    println!("Hello, world! {}", james);
}

struct Person {
    name: String,
    age: u32
}

impl std::fmt::Display for Person {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{} ({} years old)", self.name, self.age)
    }
}