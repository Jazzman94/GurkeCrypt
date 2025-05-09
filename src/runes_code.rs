use std::collections::HashMap;
use deunicode::deunicode;

pub fn to_runes(input: &str) -> String {
    let rune_map = [
        ('a', "ᚨ"), ('b', "ᛒ"), ('c', "ᚲ"), ('d', "ᛞ"), ('e', "ᛖ"),
        ('f', "ᚠ"), ('g', "ᚷ"), ('h', "ᚺ"), ('i', "ᛁ"), ('j', "ᛄ"),
        ('k', "ᚴ"), ('l', "ᛚ"), ('m', "ᛗ"), ('n', "ᚾ"), ('o', "ᛟ"),
        ('p', "ᛈ"), ('q', "ᛩ"), ('r', "ᚱ"), ('s', "ᛋ"), ('t', "ᛏ"),
        ('u', "ᚢ"), ('v', "ᚹ"), ('w', "ᚥ"), ('x', "ᛪ"), ('y', "ᛇ"),
        ('z', "ᛉ"), ('.',"."), ('!',"!"), ('?',"?"), (',',","), 
        (';',";"), (' ', " "),
    ].iter().cloned().collect::<HashMap<_, _>>();

    let cleaned: String = deunicode(input); // č → c, ř → r, ů → u, etc

    cleaned
        .to_lowercase()
        .chars()
        .filter_map(|c| rune_map.get(&c))
        .cloned()
        .collect::<String>()
}

pub fn from_runes(input: &str) -> String {
    let rune_map = [
        ("ᚨ", 'a'), ("ᛒ", 'b'), ("ᚲ", 'c'), ("ᛞ", 'd'), ("ᛖ", 'e'),
        ("ᚠ", 'f'), ("ᚷ", 'g'), ("ᚺ", 'h'), ("ᛁ", 'i'), ("ᛄ", 'j'),
        ("ᚴ", 'k'), ("ᛚ", 'l'), ("ᛗ", 'm'), ("ᚾ", 'n'), ("ᛟ", 'o'),
        ("ᛈ", 'p'), ("ᛩ", 'q'), ("ᚱ", 'r'), ("ᛋ", 's'), ("ᛏ", 't'),
        ("ᚢ", 'u'), ("ᚹ", 'v'), ("ᚥ", 'w'), ("ᛪ", 'x'), ("ᛇ", 'y'),
        ("ᛉ", 'z'), (".", '.'), ("!", '!'), ("?", '?'), (",", ','),
        (";", ';'), (" ", ' '),
    ].iter().cloned().collect::<HashMap<_, _>>();

    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == ' ' {
            result.push(' ');
            continue;
        }

        let mut rune = c.to_string();
        
        while let Some(&next_c) = chars.peek() {
            if !next_c.is_whitespace() && !rune_map.contains_key(rune.as_str()) {
                rune.push(next_c);
                chars.next();
            } else {
                break;
            }
        }

        if let Some(&latin_char) = rune_map.get(rune.as_str()) {
            result.push(latin_char);
        } else {
            result.push_str(&rune);
        }
    }

    result
}

// Testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_runes() {
        let original = "hello world";
        let runic = to_runes(original);
        let converted = from_runes(&runic);
        assert_eq!(converted, original);
    }

    #[test]
    fn test_direct_conversion() {
        assert_eq!(from_runes("ᚺᛖᛚᛚᛟ ᚥᛟᚱᛚᛞ"), "hello world");
        assert_eq!(from_runes("ᚺᛖᛚᛚᛟ!"), "hello!");
    }
}