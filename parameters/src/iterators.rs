// Rule of three translates to iterators too!
// We can have iterators over values, references or mutable references.

pub fn alt_main() {
    let mut nums = vec![1, 2, 3, 4, 5];
    for i in 1..3 {
        for j in &mut nums {
            let _: &mut u32 = j;
            println!("{},{}", i, j);
            *j *= 2;
        }
    }
}

pub fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    for i in 1..3 {
        for j in nums.iter() {
            let _: &u32 = j;
            println!("{}, {}", i, j);
        }
    }
}

pub fn main_iter() {
    let nums = vec![1, 2, 3, 4, 5];
    for i in 1..3 {
        for j in nums.iter() {
            let _: &u32 = j;
            println!("{}, {}", i, j);
        }
    }
}

pub fn main_iter_mut() {
    let mut nums = vec![1, 2, 3, 4, 5];
    for i in 1..3 {
        for j in nums.iter_mut() {
            let _: &mut u32 = j;
            println!("{}, {}", i, j);
            *j *= 2;
        }
    }
}

pub fn main_into_iter() {
    for i in 1..3 {
        let nums = vec![1, 2, 3, 4, 5];
        for j in nums.into_iter() {
            println!("{}, {}", i, j);
        }
    }
}