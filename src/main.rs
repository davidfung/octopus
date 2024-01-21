use std::io::{self, Write};

mod double_thru_channel;
mod spawn_async_tasks;

const MENU: &str = "
MENU
1. Double Through Channel
2. Spawning Async Tasks
> ";

type Task = fn();

struct MenuItem {
    id: u32,
    desc: &'static str,
    task: Task,
}

fn main() {
    let mut menuz: Vec<MenuItem> = Vec::new();
    menuz.push(MenuItem{id:1, desc:double_thru_channel::DESC, task:double_thru_channel::entry});
    menuz.push(MenuItem{id:2, desc:spawn_async_tasks::DESC, task:spawn_async_tasks::entry});

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


