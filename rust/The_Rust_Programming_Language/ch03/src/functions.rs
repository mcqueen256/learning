fn main_1() {
    println!("hello there");
    another_function(JEDI);
    multi_params(7, 146);
}

fn another_function(jedi: &str) {
    println!("Why {}, how nice it is for you to drop in.", jedi);
}

fn multi_params(x:isize, y: isize) {
    println!("x: {}, y: {}", x, y);
}

const JEDI: &str = "general konobi";

fn five() -> i32 {
    5
}

fn main() {
    let mut y = five();
    let x = y = 6;
    let x = {
        let x = 3;
        x + y
    };
    println!("X: {}", x);


}