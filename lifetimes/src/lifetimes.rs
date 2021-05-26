struct Person {
    name: String,
    age: u32,
}

/*
    Lifetimes don't have to be the same, could specify:
    get_older_name<'a, 'b>(person1: &'a Person, person2: &'b Person) -> &'a String
    but would also need to specify <'a, 'b: 'a> instead of <'a, 'b> which means that
    the resulting 'a lifetime is no longer than the 'b lifetime thus making it ok
    to take a value from person2.

    One may think that the solution below enforces that the lifetimes of all
    elements be the same. This is not the case because it simply enforces that
    all references passed to the functio must live at least as long as the reference
    returned from the function. If any individual reference happens to live longer,
    it's not a problem.
*/
fn get_older_name<'a>(person1: &'a Person, person2: &'a Person) -> &'a String {
    if person1.age >= person2.age {
        &person1.name
    } else {
        &person2.name
    }
}

pub fn main() {
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };
    let bob = Person {
        name: String::from("Bob"),
        age: 35,
    };
    let name = get_older_name(&alice, &bob);
    println!("Older person: {}", name);
}

// Multiple Lifetimes

fn message_and_return<'a, 'b>(msg: &'a String, ret: &'b String) -> &'b String {
    println!("Printing the message: {}", msg);
    ret
}

pub fn alt_main() {
    let name = String::from("Alice");
    let ret = foo(&name);
    println!("Return value: {}", ret);
}

fn foo(name: &String) -> &String {
    let msg = String::from("This is the message");
    message_and_return(&msg, &name)
}