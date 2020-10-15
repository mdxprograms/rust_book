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

fn main() {
    mutable_value();
    liftoff();
}
