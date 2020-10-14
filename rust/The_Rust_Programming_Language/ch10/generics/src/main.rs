fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest_number = largest(&number_list);
    println!("The largest number is {}", largest_number);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest_number = largest(&number_list);
    println!("The largest number is {}", largest_number);


    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

}

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     // let _integer = Point { x: 53, y:23 };
//     // let _float = Point { x: 10.5, y: 15.9 };
//     // let _mixed = Point { x: 10.4, y: 9 };

//     // let p1 = Point { x: 5, y: 10.9 };
//     // let p2 = Point { x: "Hello", y: 'c' };
//     // let p3 = p1.mixup(p2);

//     // println!("p3.x={}, p3.y={}", p3.x, p3.y);
// }