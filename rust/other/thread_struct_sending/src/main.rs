use std::thread;
use std::sync::mpsc;

#[derive(Debug)]
struct Info {
    i: u32,
    j: u32,
    k: u32
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let t = thread::spawn(move || {
        let info = Info { i: 100, j: 120, k: 420 };
        tx.send(info).unwrap();
    });
    println!("recv: {:#?}", rx.recv().unwrap());
    t.join().unwrap();

    let i: Option<i32> = None;
    println!("i: {}", i.expect("no no no"));
}
