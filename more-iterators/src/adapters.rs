struct Doubler<I> {
    iter: I
}

pub fn main() {
    let orig_iter = 1..11;
    let doubled_iter = Doubler{
        iter: orig_iter
    };
    for i in doubled_iter {
        println!("{}", i);
    }
}

pub fn alt_main() {
    for i in (1..11).map(|x| x * 2).filter(|x| x % 2 == 0) {
        println!("{}", i);
    }

    let my_vec: Vec<u32> = (1..11).collect();
    println!("{:?}", my_vec);
}

// impl<I> Iterator for Doubler<I> where
//     I: Iterator,
//     I::Item: std::ops::Mul<Output=I::Item> + From<u8>{
//     type Item = I::Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.iter.next() {
//             None => None,
//             Some(x) => Some(x * From::from(2u8))
//         }
//     }
// }

impl<I> Iterator for Doubler<I> where
    I: Iterator,
    I::Item: std::ops::Add<Output=I::Item> + Copy {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(x) => Some(x + x)
        }
    }
}