use crate::Task;
use std::io;
pub fn mark_for_completion(v: &mut Vec<Task>) {
    // enter the task number
    println!("enter the task number");
    // is it valid task number
    let mut tn: String = String::new();
    io::stdin().read_line(&mut tn);
    let tn = tn.trim().parse::<u8>();
    let counter = tn.unwrap();
    if let Some(p) = v.iter_mut().find(|c| c.tasknumber == counter) {
        p.completed = true;
        println!("task number {} completed", counter);
    } else {
        println!("invalid task number: {}", counter);
    }
}
