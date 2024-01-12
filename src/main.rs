use std::io::{self, Write};

mod double_thru_channel;
mod spawn_async_tasks;

const MENU: &str = "
MENU
1. Double Through Channel
2. Spawning Async Tasks
> ";

fn main() {
    type Task = fn();
    let id = menu();
    let g: Task;
    match id {
        1 => g = double_thru_channel::entry,
        2 => g = spawn_async_tasks::entry,
        _ => g = bye,
    }
    g();
}

fn menu() -> i32 {
    print!("{MENU}");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Input Error");
    println!("");
    match s.trim().parse::<i32>() {
        Err(_e) => {
            // println!("{}", e);
            return 0;
        }
        Ok(choice) => return choice,
    }
}

fn bye() {
    println!("Bye");
}


