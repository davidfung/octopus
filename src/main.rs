use std::io::{self, Write};

mod double_thru_channel;
mod spawn_async_tasks;
mod menu;

fn main() {
    let mut mu = menu::Menu::new();
    mu.add_item(menu::MenuItem{id:1, desc:double_thru_channel::DESC, task:double_thru_channel::entry});
    mu.add_item(menu::MenuItem{id:2, desc:spawn_async_tasks::DESC, task:spawn_async_tasks::entry});
    mu.show();

    let id = get_selection();
    let g;
    if id == 0 {
        g = bye as menu::Task;
    } else {
        g = mu.get_task_by_id(id);
    }
    g();
}

fn get_selection() -> u32 {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Input Error");
    println!("");
    match s.trim().parse::<u32>() {
        Err(e) => {
            println!("Err: {}", e);
            return 0;
        }
        Ok(choice) => return choice,
    }
}

fn bye() {
    println!("Bye");
}


