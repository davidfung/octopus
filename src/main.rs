use std::io::{self, Write};

const MENU: &str = "
MENU
1. Double Through Channel
> ";

fn main() {
    let id = menu();
    match id {
        1 => double_thru_channel(),
        _ => println!("Bye"),
    }
    type Task = fn();
    let g: Task = double_thru_channel;
    g();
}

fn menu() -> i32 {
    print!("{MENU}");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Input Error");
    println!("");
    match s.trim().parse::<i32>() {
        Err(e) => {
            println!("{}", e);
            return 0;
        }
        Ok(choice) => return choice,
    }
}

fn double_thru_channel() {
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
