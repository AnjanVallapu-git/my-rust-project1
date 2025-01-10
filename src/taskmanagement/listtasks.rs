use crate::Task;
pub fn list_task(v: &Vec<Task>) {
    for i in v {
        println!(
            "Number {:?}  Description {:?} Completion {:?}",
            i.tasknumber, i.description, i.completed
        );
    }
}
