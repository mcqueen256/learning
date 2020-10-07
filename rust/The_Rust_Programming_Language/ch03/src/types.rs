use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    // let x = 6; // using shadowing...
    x = 6;
    println!("The value of x is {}", x);

    // const MAX_POINT: u32 = 100_000;

    // let guess: u32 = "42".parse().expect("Not a number!");

    let x = 5 + 10;
    println!("The value of x is {}", x);

    let x= 10.0 / 5.1;
    println!("The value of x is {}", x);

    let c = 'z';
    let cc = 'ℤ';
    let cat = '😻';
    println!("c: {}, C: {}, Cat: {}", c, cc, cat);

    let tup: (i32, f64, u8) = (-500, 3.14, 9);
    println!("tup: ({}, {}, {})", tup.0, tup.1, tup.2);
    let (x,y,z) = tup;
    println!("tup: (x:{}, y:{}, z:{})", x, y, z);

    let months = ["January", "Febuary", "March", "April", "May", "June",
        "July","August", "September", "October", "November", "December"];
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let lx = ["this string is repeated seven times"; 7];
    // let a = [3; 5];

    println!("Please enter month number.");
    let mut month = String::new();
    io::stdin().read_line(&mut month).expect("Error reading line.");
    let month: usize = month.parse().expect("Please type a number.");
    let index = month;
    println!("this month is {}", months[index]);

}
