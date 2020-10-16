#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // area method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // can_hold method
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // square associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = User {
        email: String::from("test@test.com"),
        username: String::from("testuser"),
        active: true,
        sign_in_count: 1,
    };

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq10 = Rectangle::square(10);

    // User struct example
    println!("user: {:#?}", user1);

    // Rectangle struct example with method area
    println!(
        "The area of the rectangle is {:#?} square pixels",
        rect1.area()
    );

    // Rectangle struct example with method can_hold
    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3? {}", rect1.can_hold(&rect3));

    // Rectangle associated function square example
    println!("The square of a rectangle with {}, is {:?}", 10, sq10);
}
