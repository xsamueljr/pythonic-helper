/// Count how many times a substring appears in another string
/// 
/// # Example
/// ```
/// use pythonic_helper::strings::count;
/// 
/// let text = String::from("Hello World");
/// let substring = "l";
/// let count = count(text, substring);
/// 
/// assert_eq!(count, 3);
/// ```
pub fn count(text: String, substring: &str) -> usize {
    if text.is_empty() || substring.is_empty() {
        return 0_usize;
    }
    
    text.matches(substring).count()
}

/// Split a string into a vector of strings
/// 
/// # Example
/// ```
/// use pythonic_helper::strings::split;
/// 
/// let text = String::from("Hello World");
/// let delimiter = " ";
/// let words = split(text, Some(delimiter));
/// 
/// let expected = vec![String::from("Hello"), String::from("World")];
/// assert_eq!(words, expected);
/// assert_eq!(words.len(), 2);
/// ```
pub fn split(text: String, delimiter: Option<&str>) -> Vec<String> {
    let delimiter = delimiter.unwrap_or(" ");

    if text.is_empty() || delimiter.is_empty() {
        return vec![];
    }

    text.split(delimiter).map(|s| s.to_string()).collect()
}


#[cfg(test)]
mod tests {
    use super::count;

    #[test]
    fn test_count_empty_string() {
        let text = String::from("");
        let substring = "";

        assert_eq!(count(text, substring), 0);
    }

    #[test]
    fn test_count_no_match() {
        let text = String::from("Hello World");
        let substring = "abc";

        assert_eq!(count(text, substring), 0);
    }
}
