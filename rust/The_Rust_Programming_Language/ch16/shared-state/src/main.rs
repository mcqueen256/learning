use std::sync::Mutex;

// fn main() {
//     let m = Mutex::new(5);
//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }
//     println!("m = {:?}", m);
// }

use std::thread;
// use std::rc::Rc;

// fn main() {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     handles.into_iter().for_each(|h| h.join().unwrap());

//     println!("Result: {}", *counter.lock().unwrap());
// }

use std::sync::Arc;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("Result: {}", *counter.lock().unwrap());
}
