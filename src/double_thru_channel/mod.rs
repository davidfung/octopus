pub const DESC: &str = "Double Through Channel";

pub fn entry() {
    use std::sync::mpsc::{self, Receiver, Sender};
    use std::thread;
    use std::time;

    println!("Double a number through channel");
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    thread::spawn(move || {
        let i = rx.recv().unwrap();
        println!("{} double is {}", i, i * 2);
    });

    tx.send(2).unwrap();
    thread::sleep(time::Duration::from_millis(5));
}
