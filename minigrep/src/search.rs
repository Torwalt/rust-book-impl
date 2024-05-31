pub fn lines_containing_search<'a>(search: &str, text: &'a str) -> Vec<&'a str> {
    return text.lines().filter(|line| line.contains(search)).collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lines_containing_search() {
        let search = "Lorem";
        let text = "
Lorem ipsum dolor sit amet,
consetetur sadipscing elitr,
sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat,
sed diam voluptua Lorem.
At vero eos et accusam et justo duo dolores et ea rebum.
Stet clita kasd gubergren,
no sea takimata sanctus est Lorem ipsum dolor sit amet.
";

        let exp = vec![
            "Lorem ipsum dolor sit amet,",
            "sed diam voluptua Lorem.",
            "no sea takimata sanctus est Lorem ipsum dolor sit amet.",
        ];
        let result = lines_containing_search(&search, &text);
        assert_eq!(exp, result);
    }
}
