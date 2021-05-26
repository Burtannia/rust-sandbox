pub fn main() {
    let bytearray1: &'static [u8; 22] = b"Hello World in binary!";
    let bytearray2: &'static [u8] = b"Hello World in binary!";
    println!("{:?}", bytearray1);
    println!("{:?}", bytearray2);

    for arg in std::env::args() {
        println!(
            "arg: {}, characters: {}, bytes: {}",
            arg,
            arg.chars().count(),
            arg.bytes().count()
        );
    }
}