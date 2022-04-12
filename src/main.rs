#![allow(unused)]

use clap::Parser;
use std::fs::File;
use std::str;
use base64;
use std::io::{BufReader, Read, BufRead, Write, stdout};
use sha2::{Sha256, Sha512, Digest};

extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.
#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    title: String,
    description: String,
    repeat: bool,
    completed: bool,
}

impl Task {
    fn new() -> Self {
        Self {  title: String::new(),
                description: String::new(),
                repeat: false,
                completed: false, 
            }
    }

    fn print_task(&self) {
        println!("{}: {}", self.title, self.description);
    }

    fn define_task(&mut self, t: String, d: String, r: bool) {
        self.title = t;
        self.description = d;
        self.repeat = r;
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskList {
    tasks_vec: Vec<Task>,
    type_: String,
}

impl TaskList {
    fn new() -> Self {
        Self {  tasks_vec: Vec::new(),
                type_: String::new(), }
    }

    fn define_task_list(&mut self, v: Vec<Task>, d: String) {
        self.tasks_vec = v;
        self.type_ = d;
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct FileList {
    task_list: Vec<Task>,
}

impl FileList {
    fn new() -> Self {
        Self {  task_list: Vec::new(),
            }
    }

    fn load_file_tasks(&mut self) {
        let serialized = std::fs::read_to_string(".\\data\\tasks.json").expect("Unable to read file");
        if(serialized.len() > 0) {
            *self = serde_json::from_str(&serialized).unwrap();
        }
    }

    fn add_task(&mut self) {
        let mut t: Task = Task::new();

        let mut title = String::new();
        print!("Name of task: ");
        stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut title)
            .expect("Failed to read line");   
        let title = title.trim();

        let mut description = String::new();
        print!("Description of task: ");
        stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut description)
            .expect("Failed to read line");   
        let description = description.trim();

        let mut repeat = String::new();
        let mut r: bool;
        print!("Task repeat daily? (y/n) ");
        stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut repeat)
            .expect("Failed to read line");   
        let repeat = repeat.trim();
        if(repeat.contains("y")) {
            r = true;
        } else {
            r = false;
        }

        t.define_task((*title).to_string(), (*description).to_string(), r);
        
        self.task_list.push(t);
    }

    fn remove_task(&mut self) {
        let mut name = String::new();
        print!("Name of task: ");
        stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");   
        let name = name.trim();

        let mut count: usize = 0;
        for n in 0..self.task_list.len() {
            if(self.task_list[n].title.contains(name) && 
                self.task_list[n].title.len() == name.len()) {
                self.task_list.remove(n);
            }
        }
    }

    fn print_fl(&self) {
        println!("{:?}", self);
    }
}

const SHOW_TASKS: &str = "show tasks";
const ADD_TASK: &str = "add task";
//const ADD_TASK_LIST: &str = "add task list";
const REMOVE_TASK: &str = "remove task";
//const REMOVE_TASK_LIST: &str = "remove task list";

const QUIT: &str = "quit";

fn main() {
    //Authentication Loop
    loop {
        if (password_auth()) {
            break;
        }   
    }
    println!("Authentication Success.");

    //Load Tasks from File
    let mut global_file_list: FileList = FileList::new();
    global_file_list.load_file_tasks();

    //Program Loop
    println!("--------------------------------");
    
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");   
        let input = input.trim();

        //Command Switching
        match input {
            SHOW_TASKS => {
                global_file_list.print_fl();
            }
            ADD_TASK => {
                global_file_list.add_task();
            }
            REMOVE_TASK => {
                global_file_list.remove_task();
            }
            QUIT => { 
                break;
            }
            _ => println!("Please enter a valid command. Use \"help\" to display commands."),
        }
    }
}

fn password_auth() -> bool {
    let pw_hash = "dtnTU9OF7RjJWMhvqEPZv9mUqbg+zOaSHTC7WHNnzhA=";
    
    print!("Welcome to the productivity zone. \nPlease enter your password: ");
    stdout().flush().unwrap();
    
    let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let result = base64::encode(&result);

    if (result.contains(pw_hash)) {
        return true;
    }

    return false;
}
