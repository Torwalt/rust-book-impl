pub fn lines_containing_search(search: &String, text: &String) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    for line in text.lines() {
        if !line.contains(search) {
            continue;
        }

        out.push(String::from(line))
    }

    return out;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lines_containing_search() {
        let search = String::from("Lorem");
        let text = String::from(
            "
Lorem ipsum dolor sit amet,
consetetur sadipscing elitr,
sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat,
sed diam voluptua Lorem.
At vero eos et accusam et justo duo dolores et ea rebum.
Stet clita kasd gubergren,
no sea takimata sanctus est Lorem ipsum dolor sit amet.
",
        );

        let exp = vec![
            String::from("Lorem ipsum dolor sit amet,"),
            String::from("sed diam voluptua Lorem."),
            String::from("no sea takimata sanctus est Lorem ipsum dolor sit amet."),
        ];
        let result = lines_containing_search(&search, &text);
        assert_eq!(exp, result);
    }
}
