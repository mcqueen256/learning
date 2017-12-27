use std::fmt;

struct Structure(i32);


impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

//Implement `Display` fir `MinMax`
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use seld.number to print positionally
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64
}

//Implement `Display` fir `MinMax`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let s = Structure(70);
    println!("a structure: {}", s);

    let minmax = MinMax(-300, 300);
    println!("compare structures:");
    println!("Display {}", minmax);
    println!("Debug {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small range is {small}",
        big=big_range,
        small=small_range
    );
    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);


    // Exercise
    {
        #[derive(Debug)]
        struct Complex {
            rel: f64,
            img: f64
        }

        impl fmt::Display for Complex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} + {}", self.rel, self.img)
            }
        }

        let c = Complex { rel: 3.3, img: 7.3 };
        println!("Compare Complex:");
        println!("Display: {}", c);
        println!("Debug: {}", c);
    }
}
