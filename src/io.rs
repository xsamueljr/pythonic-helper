use std::io::{stdin, stdout, Write};


/// Provides an easy-to-use way to receive input from the user (via standard input)
///
/// # Example
/// ```no_run
/// use pythonic_helper::io::input;
/// 
/// println!("Please enter your name");
/// let name_result = input("> ");
/// let name: String = name_result.unwrap_or("default".to_owned()); 
/// ```
pub fn input(prompt: &str) -> Result<String, String> {
    let mut input: String = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    match stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(e) => Err(e.to_string()),
    }
}

/// A "yes-or-no" input. The function loops until the user inputs either "y" or "n"
///
/// # Example
/// ```no_run
/// use pythonic_helper::io::confirm;
///
/// if confirm("Do you want to continue?", None) {
///     println!("Continuing...");
/// } else {
///     println!("No? Alright then.");   
/// }
/// ```
pub fn confirm(prompt: &str, abort: Option<bool>) -> bool {
    let real_prompt = prompt.to_string() + " [y/n]: ";

    loop {
        let input_attempt = input(&real_prompt);
        if input_attempt.is_err() {
            println!("Error: {}", input_attempt.unwrap_err());
            continue;
        }
        let user_input = input_attempt.unwrap();
        if !["y", "n"].contains(&user_input.to_lowercase().as_str()) {
            println!("Error: Invalid input");
            continue;
        }

        if abort.unwrap_or(false) {
            eprintln!("Aborted.");
            std::process::exit(1);
        }

        return user_input == "y";
    }
}