use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    is_complete: bool,
}

impl Task {
    fn new(description: String) -> Self {
        Task {
            description,
            is_complete: false,
        }
    }

    fn mark_complete(&mut self) {
        self.is_complete = true;
    }
}

enum Action {
    Add(String),
    Complete(usize),
    List,
}

fn parse_args() -> Option<Action> {
    let args: Vec<String> = env::args().collect();
    match args.get(1)?.as_str() {
        "add" => {
            let description = args.get(2)?.clone();
            Some(Action::Add(description))
        }
        "complete" => {
            let index: usize = args.get(2)?.parse().ok()?;
            Some(Action::Complete(index))
        }
        "list" => Some(Action::List),
        _ => None,
    }
}

fn load_tasks(filename: &str) -> Result<Vec<Task>, serde_json::Error> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Ok(Vec::new()),
    };

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Error reading file");
    serde_json::from_str(&content).or(Ok(Vec::new()))
}

fn save_tasks(filename: &str, tasks: &Vec<Task>) -> Result<(), serde_json::Error> {
    let json = serde_json::to_string(tasks)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
        .expect("Failed to open file for writing");

    file.write_all(json.as_bytes())
        .expect("Error writing to file");
    Ok(())
}

fn main() {
    let filename = "todo_list.json";
    let mut tasks = load_tasks(filename).expect("Failed to load tasks");

    match parse_args() {
        Some(Action::Add(description)) => {
            tasks.push(Task::new(description));
            save_tasks(filename, &tasks).expect("Failed to save tasks");
            println!("Task added.");
        }
        Some(Action::Complete(index)) => {
            if let Some(task) = tasks.get_mut(index) {
                task.mark_complete();
                save_tasks(filename, &tasks).expect("Failed to save tasks");
                println!("Task {} marked as complete.", index);
            } else {
                println!("Task not found.");
            }
        }
        Some(Action::List) => {
            for (i, task) in tasks.iter().enumerate() {
                let status = if task.is_complete { "✓" } else { "✗" };
                println!("{}: [{}] {}", i, status, task.description);
            }
        }
        None => println!("Invalid command or arguments."),
    }
}
