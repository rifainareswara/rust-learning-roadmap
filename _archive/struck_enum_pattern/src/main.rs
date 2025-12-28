#[derive(Debug)]
struct Task {
    name: String,
    status: Status,
    priority: Priority
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Status {
    Backlog,
    InProgress,
    Done,
}

#[derive(Debug)]
enum Priority {
    High,
    Normal,
    Low,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    let task = Task {
        name: String::from("Go to the market"),
        status: Status::Backlog,
        priority: Priority::High,
    };

    tasks.push(task);

    let new_task = Task {
        name: String::from("Go Sleep"),
        status: Status::InProgress,
        priority: Priority::High,
    };

    tasks.push(new_task);

    println!("Backlog:");
    for task in tasks
        .iter()
        .filter(|task: &&Task | task.status == Status::Backlog)
        .collect::<Vec<&Task>>()
    {
        display_priority(&task.priority);
        println!("name: {}", task.name)
    }

    let in_progress: Vec<&Task> = tasks
        .iter()
        .filter(|task: &&Task| task.status == Status::InProgress)
        .collect();

    for (index, task) in in_progress.iter().enumerate() {
        display_priority(&task.priority);
        println!("index: {}", index);
        println!("name: {}", task.name)
    }
}

fn display_priority(priority: &Priority) {
    match priority {
        Priority::High => println!("High"),
        Priority::Normal => println!("Normal"),
        Priority::Low => println!("Low"),
    }
}
