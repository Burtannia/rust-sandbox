pub struct Empty;

pub fn main() {
    for i in Empty {
        panic!("Wait, this shouldn't happen!");
    }
    println!("All done!");
}

impl Iterator for Empty {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}