pub struct Menu {
    items: Vec<MenuItem>
}
pub struct MenuItem {
    pub id: u32,
    pub desc: &'static str,
    pub task: Task,
}

pub type Task = fn();

impl Menu {
    pub fn new() -> Menu {
        Menu{items: Vec::new()}
    }

    pub fn add_item(&mut self, item: MenuItem) {
        self.items.push(item);
    } 

    pub fn show(&self) {
        for item in &self.items {
            println!("{} {}", item.id, item.desc);
        }
    }

    pub fn get_task_by_id(&self, id: u32) -> Task {
        let pos = self.items.iter().position(|v| v.id == id).unwrap();
        self.items[pos].task
    }
}

