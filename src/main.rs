use std::env;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

impl Task{
    fn new(id: u32, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
        }
    }
    
    fn display(&self) -> String {
        let status = if self.completed { "[x]" } else { "[ ]" };
        format!("{} {}", status, self.description)
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }
}

fn load_tasks() -> Vec<Task> {
    let path = Path::new("tasks.json");

    if path.exists() {
        let data = fs::read_to_string(path).expect("Failed to read tasks.json");
        serde_json::from_str(&data).unwrap_or_else(|_| {
            println!("Warning: Failed to parse tasks.json, starting fresh.");
            Vec::new()
        })
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: & Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write("tasks.json", data).expect("Failed to write tasks.json");
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let mut tasks = load_tasks();

    match args.get(1).map(String::as_str) {
        Some("add") => {
            if args.len() > 2 {
                let description = args[2..].join(" ");
                let task = Task::new(tasks.len() as u32, description);
                tasks.push(task.clone());
                println!("Added task: {}", task.description);
            } else {
                println!("Please provide a task to add.");
            }
        }
        Some("list") => {
            for task in tasks {
                println!("{}", task.display());
            }
        }
        Some("complete") => {
            let possible_id =  args[2].parse::<u32>();
            match possible_id {
                Ok(id) => {
                    let mut found = false;
                    for task in &mut tasks {
                        if task.id == id {
                            task.mark_completed();
                            found = true;
                        }
                    }
                    if found {
                        save_tasks(&tasks);
                        println!("Task {} marked as complete.", id);
                    }
                    else {
                        println!("Task {} could not be found.", id);
                    }
                }
                Err(_) => println!("Invalid id provided."),
            }
        }
        _ => {
            println!("Invalid or missing command.");
            println!("Usage:");
            println!("  add <task>     - Add a new task");
            println!("  list           - List all tasks");
            println!("  complete <id>  - Mark a task as complete");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new(1, String::from("Buy milk"));
        assert_eq!(task.id, 1);
        assert_eq!(task.description, "Buy milk");
        assert!(!task.completed);
    }

    #[test]
    fn test_mark_complete() {
        let mut task = Task::new(2, String::from("Do laundry"));
        task.mark_completed();
        assert!(task.completed);
    }

    #[test]
    fn test_display() {
        let mut task = Task::new(3, String::from("Read book"));
        assert_eq!(task.display(), "[ ] Read book");

        task.mark_completed();
        assert_eq!(task.display(), "[x] Read book");
    }
}
