fn main() {
    println!("{}", to_pig_latin("this is a really cool string"));
}

const VOWELS: &str = "aeiou";

fn to_pig_latin(source: &str) -> String {
    let mut word = String::new();
    let mut out = String::new();
    for character in source.chars() {
        if character.is_whitespace() {
            out.push_str(&to_pig_latin_word(&word));
            out.push(' ');
            word.clear();
        } else {
            word.push(character);
        }
    }
    out.push_str(&word);
    out
}

fn to_pig_latin_word(word: &str) -> String {
    let mut first: Option<char> = None;
    let mut result = String::with_capacity(word.len() + 4);
    for (index, character) in word.char_indices() {
        if index == 0 {
            if VOWELS.contains(character) {
                first = Some('h');
                result.push(character);
            } else {
                first = Some(character)
            }
        } else {
            result.push(character);
        }
    }
    if let Some(character) = first {
        result.push('-');
        result.push(character);
        result.push_str("ay");
    }
    result
}
