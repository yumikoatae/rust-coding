// Defining the Task struct
#[derive(Debug)]
pub struct Task {
    description: String, // Description of the task
    completed: bool,     // Task completion status
}

impl Task {
    // Constructor function to create a new task
    pub fn new(description: &str) -> Task {
        Task {
            description: description.to_string(),
            completed: false,
        }
    }

    // Method to mark the task as completed
    pub fn complete(&mut self) {
        self.completed = true;
    }

    // Method to display the task information
    pub fn display(&self) -> String {
        let status = if self.completed {
            "Completed"
        } else {
            "Pending"
        };
        format!("Task: {}\nStatus: {}", self.description, status)
    }
}

// Defining the TaskManager struct to manage multiple tasks
pub struct TaskManager {
    tasks: Vec<Task>, // List of tasks
}

impl TaskManager {
    // Constructor function to create a new task manager
    pub fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    // Method to add a task to the manager
    pub fn add_task(&mut self, description: &str) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    // Method to mark a task as completed
    pub fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.complete();
        } else {
            println!("Task not found!");
        }
    }

    // Method to display all tasks
    pub fn list_tasks(&self) -> String {
        if self.tasks.is_empty() {
            "No tasks available.".to_string()
        } else {
            self.tasks
                .iter()
                .enumerate()
                .map(|(i, task)| format!("Task {}: {}", i + 1, task.display()))
                .collect::<Vec<String>>()
                .join("\n")
        }
    }
}

pub fn start_task_manager() {
    let mut manager = TaskManager::new();

    // Adding some tasks
    manager.add_task("Finish Rust homework");
    manager.add_task("Read a Rust book");
    manager.add_task("Write a blog post");

    // Displaying all tasks
    println!("Tasks List:\n{}", manager.list_tasks());

    // Completing the first task
    manager.complete_task(0);

    // Displaying the tasks again after completing one
    println!("\nUpdated Tasks List:\n{}", manager.list_tasks());
}