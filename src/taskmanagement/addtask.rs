use std::io;
pub fn add_task() -> String {
    println!("Enter Task Description");
    let mut description: String = String::new();
    io::stdin().read_line(&mut description);

    let task_description = description.trim().to_string();
    task_description
}
