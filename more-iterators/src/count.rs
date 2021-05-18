pub struct CountTen(u32);

pub fn main() {
    for i in CountTen(1) {
        println!("{}", i);
    }
    println!("All done!");
}

impl Iterator for CountTen {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 > 10 {
            None
        } else {
            let res = Some(self.0);
            self.0 += 1;
            res
        }
    }
}