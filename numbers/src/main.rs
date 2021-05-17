fn main() {
    //int_main();
    //str_main();

    //loop_ten();
    //while_ten();
    //for_ten();

    fizzbuzz();
}

fn int_main() {
    let val: i32 = 42;
    printer_int(val);
    printer_int(val);
}

fn str_main() {
    let val: String = "Hello World!".to_string();

    // printer_ref(val);
    // printer_ref(val);

    printer_ref(&val);
    printer_ref(&val);

    printer_str(val.clone());
    printer_str(val);
}

fn printer_int(val: i32) {
    println!("The value is: {}", val);
}

fn printer_str(val: String) {
    println!("The value is: {}", val);
}

// fn printer_ref(val: &String) {
//     println!("The value is: {}", val);
// }

fn printer_ref(val: &str) { // can now pass literals and don't need to force heap allocation
    println!("The value is: {}", val);
}

fn loop_ten() {
    let mut i = 1;

    loop {
        println!("i == {}", i);
        if i >= 10 {
            break
        } else {
            i += 1;
        }
    }
}

fn while_ten() {
    let mut i = 1;

    while i <= 10 {
        println!("i == {}", i);
        i += 1;
    }
}

fn for_ten() {
    for i in 1..11 {
        println!("i == {}", i)
    }
}

fn fizzbuzz() {
    for i in 1..101 {
        match(i % 3 == 0, i % 5 == 0) {
            (true, true) => println!("FizzBuzz"),
            (true, _) => println!("Fizz"),
            (_, true) => println!("Buzz"),
            (false, false) => println!("{}", i),
        }
    }
}