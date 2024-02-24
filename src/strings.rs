pub fn count(text: String, substring: &str) -> usize {
    text.matches(substring).count() as usize
}


#[cfg(test)]
mod tests {
    use super::count;

    #[test]
    fn test_count() {
        let text = String::from("Hello World");
        let substring = "l";

        assert_eq!(count(text, substring), 3);
    }
}