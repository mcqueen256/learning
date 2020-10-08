// hello world

// fn main() {
//     let _lucky_number = 7; // this is a comment

//     /* multi
//     line
//     comment */

//     let number = 3;
//     if number != 0 {
//         println!("condition was true!");
//     } else {
//         println!("condition was false!");
//     }

//     let condition = true;
//     let x = if condition { 5 } else { "three".parse::<u32>().expect("Not a number") };
// }

fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 10;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // looping through a collection
    let collection = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("The value at index {} is {}.", index, collection[index]);
        index += 1;
    }

    // a better way?
    for element in collection.iter() {
        println!("the value is {}", element);
    }

    // to have the index as well
    for (i, element) in collection.iter().enumerate() {
        println!("The value at index {} is {}.", i, element);
    }

    // using functional programming?
    collection.iter()
        .enumerate()
        .map(|(i,e)| format!("The value at index {} is {}.", i ,e))
        .for_each(|s| println!("{}", s));

    for number in (0..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");

}