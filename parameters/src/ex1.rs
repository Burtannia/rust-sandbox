fn double(x: &mut u32) {
    *x *= 2;
}

pub fn main() {
    let mut x = 5;
    double(&mut x);
    println!("{}", x);
}