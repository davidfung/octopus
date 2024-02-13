// module usage:
//
// use crate::menu::MenuItem;
//
// pub fn menu() -> MenuItem {
//     MenuItem {
//         task: entry,
//         desc: "Module Description...",
//     }
// }
//
// pub fn entry() {
//     todo!();
// }

use std::collections::BTreeMap;

pub struct Menu {
    items: BTreeMap<u32, MenuItem>,
}

pub struct MenuItem {
    pub task: fn(),
    pub desc: &'static str,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            items: BTreeMap::new(),
        }
    }

    pub fn add_item(&mut self, id: u32, item: MenuItem) {
        self.items.insert(id, item);
    }

    pub fn show(&self) {
        for (id, item) in &self.items {
            println!("{} {}", id, item.desc);
        }
    }

    pub fn get_task_by_id(&self, id: u32) -> Option<fn()> {
        if let Some(item) = self.items.get(&id) {
            Some(item.task)
        } else {
            None
        }
    }
}
