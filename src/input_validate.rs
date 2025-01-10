use std::io;
pub fn enter_validate_input() -> Result<u8, String> {
    // enter the input
    let mut input_option: String = String::new();
    println!("choose an Option");
    println!("1. Add a task");
    println!("2. List tasks");
    println!("3. Mark a task as complete");
    println!("4. Exit program");
    io::stdin().read_line(&mut input_option);
    // validate the input
    let selected_option_as_int = input_option.trim().parse::<u8>();
    match selected_option_as_int {
        Ok(val) if (1..=4).contains(&val) => Ok(val),
        Ok(_) => return Err(String::from(" Invalid input valid range 1 to 4")),
        Err(err) => {
            let err1 = String::from(" Only integer numbers are allowed") + &err.to_string();
            return Err(err1);
        }
    }
}
