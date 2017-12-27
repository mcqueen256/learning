fn main() {
    struct UnPrintable(i32);

    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Deep(DebugPrintable);

    println!("{:?}", 12); // std library types can also be printed with {:?}
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's");
    println!("Noe {:?} will print!", DebugPrintable(34));
    println!("Noe {:?} will print!", Deep(DebugPrintable(76)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    //pretty print
    println!("{:#?}", peter);
}
