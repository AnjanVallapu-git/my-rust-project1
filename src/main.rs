mod input_validate;
mod taskmanagement;
use std::process;
#[derive(Debug)]
pub struct Task {
    tasknumber: u8,
    description: String,
    completed: bool,
}

use input_validate::enter_validate_input;
use taskmanagement::addtask::add_task;
use taskmanagement::listtasks::list_task;
use taskmanagement::markcompletion::mark_for_completion;

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut tasknumber: u8 = 0;
    while true {
        let input = enter_validate_input();
        if let Err(err) = input {
            println!("Error{}", err);
            process::exit(1);
        }
        let p = input.unwrap();
        match p {
            1 => {
                tasknumber = tasknumber + 1;
                let t = add_task();
                tasks.push(Task {
                    tasknumber,
                    description: t,
                    completed: false,
                });
            }
            2 => list_task(&tasks),
            3 => mark_for_completion(&mut tasks),
            4 => break,
            _ => process::exit(2),
        }
    }
}
