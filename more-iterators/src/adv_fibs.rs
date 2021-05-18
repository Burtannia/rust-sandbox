enum Fibs{
    Done,
    OneLeft(u32),
    Running(u32, u32)
}

pub fn main() {
    for i in Fibs::Running(0, 1) {
        println!("{}", i);
    }
    println!("All done!");
}

impl Iterator for Fibs {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        use Fibs::*;

        match *self {
            Done => None,
            OneLeft(x) => {
                *self = Done;
                Some(x)
            },
            Running(x,y) => {
                *self = match x.checked_add(y) {
                    None => OneLeft(y),
                    Some(newY) => Running(y, newY)
                };

                Some(x)
            }
        }
    }
}