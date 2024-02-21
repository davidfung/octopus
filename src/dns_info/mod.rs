use crate::menu::MenuItem;

pub fn menu() -> MenuItem {
    MenuItem {
        task: entry,
        desc: "Get DNS TXT Record",
    }
}

pub fn entry() {
    todo!();
}
