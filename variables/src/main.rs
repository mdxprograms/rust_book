fn mutable_value() {
    println!("Mutable number for x");

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn liftoff() {
    println!("loop numbers in reverse and take off");

    for number in (1..6).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!");
}

// mutable reference example
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn main() {
    mutable_value();
    liftoff();

    let mut s = String::from("hello");

    println!("s was {}", s);

    change(&mut s);

    println!("s is {}", s);
}
