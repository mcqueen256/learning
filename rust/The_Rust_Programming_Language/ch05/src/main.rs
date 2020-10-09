// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// struct Colour(i32, i32, i32);
// struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


fn main() {
    // {
    //     let mut user1 = build_user(String::from("someone@example.com"), String::from("someusername1234"));
    //     user1.email = String::from("anotheremail@example.com");

    //     // Using rust stuct update syntax
    //     let user2 = User {
    //         email: String::from("another@example.com"),
    //         username: String::from("another"),
    //         ..user1
    //     };

    //     let black = Colour(0,0,0);
    //     let origin = Point(0,0,0);
    // }

    // { // okay, but a bit much to write
    //     let width1 = 30;
    //     let height1 = 50;
    //     println!(
    //         "The area of the rectangle is {} square pixels.",
    //         area(width1, height1)
    //     );
    // }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10, height: 40,
        };
        let rect3 = Rectangle {
            width: 60, height: 40,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );

        println!("rect1 is {:#?}", rect1);
        
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

        let square = Rectangle::square(70);
        println!("Made a square: {:?}", square);
    }
}


// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}