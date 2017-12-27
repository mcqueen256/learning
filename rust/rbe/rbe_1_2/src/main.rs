fn main() {
    // Print macros defined in std::fmt
    /*

    format! produces a string from a format string spec
    print! prints a format spec to stdout without a newline
    println! prints a format spec to stdout with a newline
    eprint! same as print! but prints to stderr
    eprintln! same as println! but to stderr
    */

    println!("ex 0: one substitue: {}", "Hello there");
    println!("ex 1: multiple substitutions: {} {}", "there are two variables", 42);
    println!("ex 2: use order {0} {1} {1} {0}", 79, 24);
    println!("ex 3: named: {foo} {bar} {bax} {low}",
        bax="this",
        foo="foo",
        bar="that",
        low="the other"
    );
    println!("ex 4: binary (special formating specified) {0} {0:b}", 6);
    println!("ex 5: #{number:>width$}#", number=1, width=6);
    println!("ex 6: #{number:<width$}#", number=1, width=6);
    println!("ex 7: #{number:>0width$}#", number=1, width=6);


    /*
    When prining or formatting, use {} to print things that implement the fmt::Display trait and
    {:?} for things that things that implement the fmt::Debug trait
    */

    // exercises
    println!("My name is {0}, {1} {0}", "Bond", "James");
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);
    println!("This struct `{:?}` will print...", Structure(3));
    let pi = 3.141592;
    println!("{pi:.3}", pi=pi);
    // println!("ex : {}", );
    // println!("ex : {}", );
    // println!("ex : {}", );
    // println!("ex : {}", );
    // println!("ex : {}", );
    // println!("ex : {}", );
    // println!("ex : {}", );
    // println!("ex : {}", );
    // println!("ex : {}", );
    // println!("ex : {}", );
    // println!("ex : {}", );
}
