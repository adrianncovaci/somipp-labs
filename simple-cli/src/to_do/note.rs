pub enum Command {
    Get,
    Add(String),
    Delete(usize),
    Complete(usize),
}

struct ToDoItem {
    name: String,
    completed: char,
}

impl ToDoItem {
    fn new(name: String) -> ToDoItem {
        ToDoItem {
            name,
            completed: ' ',
        }
    }
}

pub struct ToDoList {
    list: Vec<ToDoItem>
}

impl ToDoList {
    pub fn new() -> ToDoList {
        ToDoList { list: vec![] }
    }

    pub fn create_to_do(&mut self, name: String) {
        let new_it = ToDoItem::new(name);
        &self.list.push(new_it);
    }

    pub fn complete_to_do(&mut self, index: usize) {
        self.list[index-1].completed = 'X';
    }

    pub fn delete_to_do(&mut self, index: usize) {
        &self.list.remove(index-1);
    }

    pub fn display_list(&self) {
        for i in 0..self.list.len() {
            println!("{}) [{}] - {}", i+1, self.list[i].completed, self.list[i].name);
        }
    }
}
