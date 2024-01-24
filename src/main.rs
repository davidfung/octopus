use std::io::{self, Write};

mod double_thru_channel;
mod spawn_async_tasks;

type Task = fn();

struct MenuItem {
    id: u32,
    desc: &'static str,
    task: Task,
}

fn main() {
    let mut menu: Vec<MenuItem> = Vec::new();
    menu.push(MenuItem{id:1, desc:double_thru_channel::DESC, task:double_thru_channel::entry});
    menu.push(MenuItem{id:2, desc:spawn_async_tasks::DESC, task:spawn_async_tasks::entry});

    show_menu(&menu);
    let id = get_selection();
    let g;
    if id == 0 {
        g = bye as Task;
    } else {
        let pos = menu.iter().position(|v| v.id == id).unwrap();
        g = menu[pos].task;
    }
    g();
}

fn show_menu(menu: &Vec<MenuItem>) {
    for item in menu {
        println!("{} {}", item.id, item.desc);
    }
}

fn get_selection() -> u32 {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Input Error");
    println!("");
    match s.trim().parse::<u32>() {
        Err(e) => {
            println!("{}", e);
            return 0;
        }
        Ok(choice) => return choice,
    }
}

fn bye() {
    println!("Bye");
}


