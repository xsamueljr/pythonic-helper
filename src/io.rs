use std::io::{stdin, stdout, Write};


/// Provides an easy-to-use way to receive input from the user (via standard input)
///
/// # Example
/// ```
/// use pythonic_helper::io::input;
/// 
/// fn main() {
///     println!("Please enter your name");
///     let name: String = input("> "); 
/// }
/// ```
/// 
/// 
pub fn input(prompt: &str) -> String {
    let mut input: String = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("invalid input");

    input.trim().to_string()
}

pub fn confirm(prompt: &str, abort: Option<bool>) -> bool {
    let real_prompt = prompt.to_string() + " [y/n]";

    loop {
        let user_input: String = input(&real_prompt);
        if !["y", "n"].contains(&user_input.as_str()) {
            println!("Error: Invalid input");
            continue;
        }

        if !abort.unwrap_or(false) {
            panic!("Aborted");
        }

        return user_input == "y";
    }
}