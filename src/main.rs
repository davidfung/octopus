use std::io::{self, Write};

mod double_thru_channel;
mod dns_info;
mod get_version;
mod json_macro;
mod spawn_async_tasks;
mod menu;
pub mod version;

fn main() {
    let mut mu = menu::Menu::new();
    mu.add_item(1, double_thru_channel::menu());
    mu.add_item(2, dns_info::menu());
    mu.add_item(3, spawn_async_tasks::menu());
    mu.add_item(4, json_macro::menu());
    mu.add_item(5, get_version::menu());
    mu.show();

    let id = get_selection();
    let g = mu.get_task_by_id(id);
    match g {
        Some(g) => g(),
        _ => bye(),
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
            println!("Err: {}", e);
            return 0;
        }
        Ok(choice) => return choice,
    }
}

fn bye() {
    println!("Bye");
}


