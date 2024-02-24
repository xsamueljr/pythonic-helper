use std::io::{stdin, stdout, Write};


/// Provides an easy-to-use way to receive input from the user (via standard input)
///
/// # Example
/// ```no_run
/// use pythonic_helper::io::input;
/// 
/// println!("Please enter your name");
/// let name: String = input("> "); 
/// ```
pub fn input(prompt: &str) -> String {
    let mut input: String = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("invalid input");

    input.trim().to_string()
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
        let user_input: String = input(&real_prompt);
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