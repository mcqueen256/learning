use std::thread;
use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(||{
//         for i in 0..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     handle.join().unwrap();
// }


// fn main() {
//     let v = vec![1,2,3,4];
//     let handle = thread::spawn(move || {
//         println!("here is  a vector {:?}", v);
//     });

//     handle.join().unwrap();
// }

use std::sync::mpsc;

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     let handle = thread::spawn(move || {
//         let val = String::from("hello");
//         tx.send(val).unwrap();
//     });
//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
// }

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("there"),
            String::from("from"),
            String::from("thread"),
        ];
        vals.into_iter().for_each(|v| {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(2));
        });
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        vals.into_iter().for_each(|v| {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(2));
        });
    });
    // while let Ok(val) = rx.recv() {
    //     println!("Got: {}", val);
    // }
    for val in rx {
        println!("Got: {}", val);
    }
    println!("Done.");
}