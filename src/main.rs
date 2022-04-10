#![allow(unused)]

use clap::Parser;
use std::fs::File;
use std::str;
use base64;
use std::io::{BufReader, Read, BufRead};
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
        Self { title: String::new(),
                description: String::new(),
                repeat: false,
                completed: false, }
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
        Self { tasks_vec: Vec::new(),
                type_: String::new(), }
    }

    fn define_task_list(&mut self, v: Vec<Task>, d: String) {
        self.tasks_vec = v;
        self.type_ = d;
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct FileList {
    lists_vec: Vec<TaskList>,
}

impl FileList {
    fn new() -> Self {
        Self { lists_vec: Vec::new() }
    }

    fn load_file_tasks(&mut self) {
        let serialized = std::fs::read_to_string(".\\data\\tasks.json").expect("Unable to read file");
        if(serialized.len() > 0) {
            *self = serde_json::from_str(&serialized).unwrap();
        }
    }

    fn test_print_fl(&self) {
        println!("FileList = {:?}", self);
    }
}

const SHOW_TASKS: &str = "show tasks";
const ADD_TASK: &str = "add task";
const REMOVE_TASK: &str = "remove task";
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
                //println!("{} , {}", input, show_tasks);
                global_file_list.test_print_fl();
            }
            ADD_TASK => {

            }
            REMOVE_TASK => {

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
