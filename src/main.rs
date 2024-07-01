use std::{
    env,
};

struct TodoItem {
    name: String,
    completed: char,
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem {
            name,
            completed: ' ',
        };
    }
}
struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        return TodoList { list: Vec::new() };
    }

    fn add_to_list(&mut self, name: String) {
        let todo_item = TodoItem::new(name);
        let _ = self.list.push(todo_item);
    }

    fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("[{}] - {}", item.completed, item.name);
        }
    }
    fn mark_done(&mut self, index: usize) {
        if (self.list[index].completed) == ' ' {
            self.list[index].completed = 'x';
        } else {
            self.list[index].completed = ' '
        }
    }

    fn remove_task(&mut self, index: usize) {
        self.list.remove(index);
    }
}

enum Command {
    get,
    add(String),
    done(usize),
    remove(usize),
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut todo_list = TodoList::new();

    let command = match arguments[1].as_str() {
        "get" => Command::get,
        "add" => Command::add(arguments[2].clone()),
        "done" => Command::done(arguments[2].parse().unwrap()),
        "remove" => Command::remove(arguments[2].parse().expect("error parsing for a remove")),
        _ => panic!("you must provide an accepted command"),
    };

    todo_list.add_to_list("My name is Emeke".to_string());
    todo_list.add_to_list("I am a Senior Software Engineer with 5 years of experience".to_string());

    match command {
        Command::get => todo_list.print(),
        Command::add(task) => {
            todo_list.add_to_list(task);
            todo_list.print();
        }
        Command::done(index) => {
            todo_list.mark_done(index);
        }
        Command::remove(index) => {
            todo_list.remove_task(index);
            todo_list.print()
        }
    }
}
