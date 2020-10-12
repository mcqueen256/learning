use std::fs::File;
use std::io::ErrorKind;

// fn main() {
//     let f = File::open("hello.txt");

//     let f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(error) => panic!("Problem creating the file: {:?}", error)
//             },
//             other_error => panic!("Problem opening the file: {:?}", other_error)
//         }
//     };
// }

// fn main() {
//     let f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error)
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error)
//         }
//     });
// }

// fn main() {
//     // let f = File::open("hello.txt").unwrap();
//     let f = File::open("hello.txt").expect("Failed to open hello.txt");
// }

use std::io;
use std::io::Read;

fn _read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn _read_text_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn _even_shorter_read_text_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


use std::fs;
fn _extreme_short_version_of_the_read_text_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// fn main() {
//     println!("text: {}", read_username_from_file().expect("Failed to read from file."));
// }

use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let _f = File::open("hello.txt")?;
    Ok(())
}