/*
Rust will try to capture variables in the weakest form possible.
Immutable reference, then mutable reference, then by value:
- If any part of the closure uses a variable by value, it must be captured by value.
- Otherwise, if any part of the closure uses a variable by mutable reference, it must be captured
    by mutable reference.
- Otherwise, if any part of the closure uses a variable by immutable reference, it must be
    captured by immutable reference.

Rust does however, only look at the body of the closure itself
rather than the full context. So this can cause lifetime errors.

`move` forces capture of the local scope by value.

If a closure uses anything by value, it is FnOnce.
Otherwise if it uses anything by mutable reference, it is FnMut,
which automatically implies FnOnce.
Otherwise it is Fn, which implies FnMut and FnOnce.
*/

fn call_fn<F>(f: F) where F: Fn() {
    f()
}

fn call_fn_mut<F>(mut f: F) where F: FnMut() {
    f()
}

fn call_fn_once<F>(f: F) where F: FnOnce() {
    f()
}

pub fn main() {
    let say_hi = {
        let name = String::from("Alice");
        move || println!("Hello, {}", name)
    };
    call_fn(&say_hi);
    call_fn_mut(&say_hi);
    call_fn_once(&say_hi);
}

pub fn main_mut() {
    let mut say_hi = {
        let mut name = String::from("Alice");
        move || {
            name += " and Bob";
            println!("Hello, {}", name);
        }
    };
    //call_fn(say_hi);
    call_fn_mut(&mut say_hi);
    call_fn_once(&mut say_hi);
}