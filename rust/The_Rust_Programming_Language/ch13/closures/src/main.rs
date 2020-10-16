use std::thread;
use std::time;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> (u32)
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    
    // let _expensive_closure = |num: i32| -> i32 {
    //     println!("calculating slowly...");
    //     thread::sleep(time::Duration::from_secs(2));
    //     num
    // };

    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello there"));
    // let n = example_closure(5);


}
