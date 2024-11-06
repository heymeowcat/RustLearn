#[derive(Debug)]
enum TaskStatus {
    Todo,
    InProgress,
}

#[derive(Debug)]
struct Task {
    id: u32,
    description: String,
    status: TaskStatus,
}

struct TaskTracker {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskTracker {
    fn new() -> Self {
        TaskTracker {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task {
            id: self.next_id,
            description,
            status: TaskStatus::Todo,
        };
        self.tasks.push(task);
        self.next_id += 1;
    }

    fn update_task_status(&mut self, id: u32, new_status: TaskStatus) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = new_status;
            Ok(())
        } else {
            Err(format!("Task with id {} not found", id))
        }
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "ID: {}, Description: {}, Status: {:?}",
                task.id, task.description, task.status
            );
        }
    }
}

fn main() {
    let mut tracker = TaskTracker::new();
    tracker.add_task("Task one".to_string());
    tracker.add_task("Task two".to_string());

    tracker
        .update_task_status(1, TaskStatus::InProgress)
        .unwrap();
    tracker.list_tasks();
}
