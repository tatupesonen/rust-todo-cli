use colored::*;
use std::env;
use std::cmp::*;
use std::fs::*;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::os;
use serde::{Deserialize, Serialize};


#[derive(Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
struct TodoItem {
    priority: Priority,
    description: String,
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
enum Priority {
    High,
    Medium,
    Low,
}

impl TodoItem {
    fn print_item(&self, i: &i32) {
        match self.priority {
            Priority::High => println!("{}: {} {}", i, "High".red(), self.description),
            Priority::Medium => println!("{}: {} {}", i, "Medium".green(), self.description),
            Priority::Low => println!("{}: {} {}", i, "Low".blue(), self.description),
        }
    }
}

impl Ord for TodoItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn sort(&mut self) {
        self.items.sort();
    }
    fn add(&mut self, item: TodoItem) {
        self.items.push(item);
    }
    fn create_item(&mut self, priority_string: String, description: String) {
        let priority: Priority = match &priority_string[..] {
            "high" => Priority::High,
            "medium" => Priority::Medium,
            "low" => Priority::Low,
            _ => Priority::Low,
        };
        
        self.items.push(TodoItem {
            description: description,
            priority: priority
        });
    }
    fn delete_item(&mut self, index_as_str: String) {
        let index = index_as_str.parse::<i32>().unwrap() - 1;
        if index as usize <= self.items.len() {
            self.items.remove(index as usize);
        }
    }
    fn print_list(&self) {
        println!("Todo-list:");
        println!("------------------------------------");
        let mut counter: i32 = 0;
        for i in &self.items {
            counter = counter + 1;
            i.print_item(&counter)
        }
    }
}

fn check_args(list: &mut TodoList) {
    let args: Vec<String> = env::args().collect();

    //check if we got enough args
    if args.len() >= 2 {

        match &args[1][..] {
            "add" => list.create_item(args[2].clone(), args[3].clone()),
            "delete" => list.delete_item(args[2].clone()),
            _ => println!("anything"),
        }
    }
}

fn check_folder(path: &PathBuf) {
    fs::create_dir_all(path).unwrap();
}

fn start(args: String, path: &PathBuf) -> TodoList {
    //check if file exists
    let res = fs::read_to_string(&path);
    match res {
        Ok(string) => {
            let list = serde_json::from_str(&string);
            let list = match list {
                Ok(list) => list,
                Err(e) => TodoList {
                    items: Vec::new(),
                }
            };
            list
        }
        //File doesn't exist yet
        Err(e) => {
            println!("Couldn't read todo list file. Creating a new list... {}", e);
            let list = TodoList {
                items: Vec::new(),
            };
            save(&path, &list);
            list
        }
    }
}

fn save(path: &PathBuf, todolist: &TodoList) {
    let json = serde_json::to_string(todolist).unwrap();
    let mut file = File::create(&path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn main() {
    //create path
    let mut path = std::env::home_dir().unwrap();
    let mut folder_path = path.clone();
    folder_path.push(".rust_todo");
    check_folder(&folder_path);
    let mut path = folder_path.clone();
    path.push("list.json");
    
    let args: Vec<String> = env::args().collect();
    let mut todo_list = start(args[0].clone(), &path);

    check_args(&mut todo_list);
    todo_list.sort();
    todo_list.print_list();
    save(&path, &todo_list);
}
