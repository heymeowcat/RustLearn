use std::env;

#[derive(Debug)]
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

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    match parse_args() {
        Some(Action::Add(description)) => {
            tasks.push(Task::new(description));
            println!("Task added.");
        }
        Some(Action::Complete(index)) => {
            if let Some(task) = tasks.get_mut(index) {
                task.mark_complete();
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
