pub fn main() {
    call_with_hi(say_message);
    call_with_hi(say_message);
}

fn say_message(msg: &str) {
    println!("{}", msg)
}

fn call_with_hi<F>(f: F)
    where F: Fn(&str) {
    f("Hi!");
}

fn call_with_bye<F>(f: F)
    where F: Fn(&str) {
    f("Bye!");
}

pub fn main_alt() {
    let name = String::from("Alice");
    let say_something = |msg: &str| println!("{}, {}", msg, name);
    call_with_hi(say_something);
    call_with_hi(say_something);
    call_with_bye(say_something);
    call_with_bye(say_something);
}