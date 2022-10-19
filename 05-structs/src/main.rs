#![allow(unused)]


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    // let user2 = User{
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count
    // };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
    {
        fn area(width: u32, height: u32) -> u32 {
            width * height
        }
        println!(
            "The area of the rectangle is {} square pixels.",
            area(30, 50)
        );
    }

    {
        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }
        let rect1 = (30, 50);
        println!(
            "The area of the rectangle is {} square pixels.",
            area(rect1)
        );
    }
    {
        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );

        println!("rect1 is {:#?}", rect1);
        dbg!(&rect1);
  
    }
}
