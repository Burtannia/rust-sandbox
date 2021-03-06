pub fn main() {
    for i in TheAnswer.take(10) {
        println!("The answer to life, the universe, and everything is {}", i);
    }
    println!("All done!");
}

pub struct TheAnswer;

impl Iterator for TheAnswer {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(42)
    }
}