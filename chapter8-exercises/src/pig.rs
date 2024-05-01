use std::collections::HashMap;

pub fn to_pig_latin(input: String) -> String {
    let mut new_str = String::new();
    let consonat_addon = "ay";
    let vowel_addon = "hay";
    let vowels = HashMap::from([('a', 0), ('e', 0), ('i', 0), ('o', 0), ('u', 0)]);
    let punctuation = ['.', ',', ':', ';'];

    for mut word in input.split_whitespace() {
        word = dbg!(word);

        // Handle punctuation.
        let has_punctuation = word.ends_with(punctuation);
        let mut safe_punc = String::new();
        if has_punctuation {
            let mut c = word.chars();
            let last_char = c.next_back();
            if let Some(last_char) = last_char {
                safe_punc.push(last_char);
            };

            word = c.as_str();
        }

        let first_char = match word.chars().next() {
            Some(c) => c,
            None => continue,
        };
        new_str = dbg!(new_str);

        let first_char_is_vowel = vowels.contains_key(&first_char.to_ascii_lowercase());
        if first_char_is_vowel {
            let new_word = format!(" {}-{}{}", word, vowel_addon, safe_punc);
            new_str.push_str(&new_word);
            new_str = dbg!(new_str);
            continue;
        }

        // Else do the consonat_addon
        let mut word_chars = word.chars();
        word_chars.next();
        let first_char_cut = word_chars.as_str();

        let new_word = format!(
            " {}-{}{}{}",
            first_char_cut, first_char, consonat_addon, safe_punc
        );
        new_str.push_str(&new_word);
        new_str = dbg!(new_str);
    }

    return new_str;
}
