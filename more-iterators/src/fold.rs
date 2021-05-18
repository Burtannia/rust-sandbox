pub fn main() {
    let range = 1..11;
    let i = sum(range);
    println!("{}", i);
}

fn sum<F>(f: F) -> F::Item where
    F: Iterator,
    F::Item: std::ops::Add<Output=F::Item> + From<u8>{
    f.fold(From::from(0u8), |x, total| x + total)
}