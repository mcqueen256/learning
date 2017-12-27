use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

struct ListX(Vec<i32>);

impl fmt::Display for ListX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[");
        for (i, elm) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", i, elm)?;
        }
        write!(f, "]")
    }
}


fn main() {
    let v = List(vec![1, 2, 3]);
    println!("v: {}", v);
    let x = ListX(vec![1,0,2,9,3,8,4,7,5,6]);
    println!("x: {}", x);
}
