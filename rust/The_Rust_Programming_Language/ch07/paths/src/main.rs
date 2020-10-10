use std::collections::HashMap;
use std::io::Result as IoResult;
use std::fmt::Result;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("Hello, world!");
}

fn function1() -> Result {
    // --snipet --
}

fn function2() -> IoResult<()> {
    // --snipet --
}
