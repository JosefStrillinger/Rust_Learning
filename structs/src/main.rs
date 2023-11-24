// A Tuple can be used as an alternative to a struct
struct Color(i32, i32, i32);

// Creating a struct
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// How to print struct using debug
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// How to add a method to a struct
impl Rectangle {
    fn area(&self) -> u32 { // when method is defined with sself it is accesses with point .
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.area() >= rect.area()
    }
    fn square(size: u32) -> Self { // associated method, often used, when it creates new instances, accessed with ::
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Initiating a struct
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    dbg!(&user1);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // implements everything else
    };

    let _user = build_user(
        String::from("another@example.com"),
        String::from("another@example.com"),
    );

    println!("{} {} {} {}", user2.username, user2.email, user2.sign_in_count, user2.active);

    let black = Color(0, 0, 0);
    println!("{}", black.0);

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
    dbg!(&rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    dbg!(Rectangle::square(2));
}

// Function to build a struct
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
