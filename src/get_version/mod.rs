use crate::menu::MenuItem;

pub fn menu() -> MenuItem {
    MenuItem {
        task: entry,
        desc: "Get build number",
    }
}

pub fn entry() {
   println!("Build no. = {}", super::version::VERSION);
}