use std::collections::HashMap;
use deunicode::deunicode;

// Definice mapování run - pouze jednou
const RUNES: &[(char, &str)] = &[
    ('a', "ᚨ"), ('b', "ᛒ"), ('c', "ᚲ"), ('d', "ᛞ"), ('e', "ᛖ"),
    ('f', "ᚠ"), ('g', "ᚷ"), ('h', "ᚺ"), ('i', "ᛁ"), ('j', "ᛄ"),
    ('k', "ᚴ"), ('l', "ᛚ"), ('m', "ᛗ"), ('n', "ᚾ"), ('o', "ᛟ"),
    ('p', "ᛈ"), ('q', "ᛩ"), ('r', "ᚱ"), ('s', "ᛋ"), ('t', "ᛏ"),
    ('u', "ᚢ"), ('v', "ᚹ"), ('w', "ᚥ"), ('x', "ᛪ"), ('y', "ᛇ"),
    ('z', "ᛉ"), ('.', "."), ('!', "!"), ('?', "?"), (',', ","),
    (';', ";"), (' ', " "),
];

pub fn to_runes(input: &str) -> String {
    let rune_map: HashMap<char, &str> = RUNES.iter().copied().collect();
    let cleaned = deunicode(input);
    
    cleaned
        .to_lowercase()
        .chars()
        .filter_map(|c| rune_map.get(&c).copied())
        .collect::<String>()
}

pub fn from_runes(input: &str) -> String {
    let rune_map: HashMap<&str, char> = RUNES.iter()
        .map(|(latin, rune)| (*rune, *latin))
        .collect();
    
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