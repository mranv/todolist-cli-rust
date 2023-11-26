use std::io;
use std::io::Write;

struct Todo {
    list: Vec<String>,
}

impl Todo {
    fn new() -> Todo {
        Todo {
            list: Vec::new(),
        }
    }

    fn add(&mut self, item: String) {
        self.list.push(item);
    }

    fn complete(&mut self, index: usize) {
        if index < self.list.len() {
            self.list.remove(index);
        } else {
            println!("Invalid index.");
        }
    }

    fn print(&self) {
        for (i, item) in self.list.iter().enumerate() {
            println!("{}. {}", i + 1, item);
        }
    }
}

fn main() {
    let mut todo = Todo::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        match input {
            "add" | "a" => {
                print!("Enter task: ");
                io::stdout().flush().unwrap();
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                todo.add(task.trim().to_string());
            }
            "complete" | "c" => {
                print!("Enter index to complete: ");
                io::stdout().flush().unwrap();
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).unwrap();
                let index: usize = index_str.trim().parse().unwrap();
                todo.complete(index - 1); 
            }
            "print" | "p" => todo.print(),
            "quit" | "q" => break,
            _ => println!("Invalid command."),
        }
    }
}