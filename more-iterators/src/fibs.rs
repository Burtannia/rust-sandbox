struct Fibs(u32, u32);

pub fn main() {
    for i in Fibs(0, 1) {
        println!("{}", i);
    }
    println!("All done!");
}

impl Iterator for Fibs {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.checked_add(self.1) {
            None => None,

            Some(x) => {
                let res = self.0;
                self.0 = self.1;
                self.1 = x;
                Some(res)
            }
        }
    }
}