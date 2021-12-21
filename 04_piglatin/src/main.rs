fn main() {
    println!("{}", to_pig_latin("this is a  really cool string "));
}

const VOWELS: &str = "aeiou";

fn to_pig_latin(source: &str) -> String {
    let mut out = String::with_capacity(source.len()); // will have at least the size of source
    let mut word_start = 0;
    let mut word_end = 0;
    for (index, character) in source.char_indices() {
        if character.is_whitespace() {
            out.push_str(&to_pig_latin_word(&source[word_start..word_end]));
            out.push(' ');
            word_start = index + 1;
            word_end = word_start;
        } else {
            word_end += 1;
        }
    }
    out.push_str(&to_pig_latin_word(&source[word_start..word_end]));
    out
}

fn to_pig_latin_word(word: &str) -> String {
    let mut result = String::with_capacity(word.len() + 4);
    let first = word.chars().next().map(|character| {
        if VOWELS.contains(character) {
            result.push(character);
            'h'
        } else {
            character
        }
    });
    for character in word.chars().skip(1) {
        result.push(character);
    }
    if let Some(character) = first {
        result.push('-');
        result.push(character);
        result.push_str("ay");
    }
    result
}
