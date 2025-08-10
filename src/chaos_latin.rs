use std::collections::HashMap;
use deunicode::deunicode;

const CIPHER: &[(char, &str)] = &[
    ('a', "@"), ('b', "z"), ('c', "&"), ('d', "9"), ('e', "#"),
    ('f', "$"), ('g', "3"), ('h', "x"), ('i', "%"), ('j', "8"),
    ('k', "^"), ('l', "w"), ('m', "2"), ('n', "v"), ('o', "7"),
    ('p', "~"), ('q', "u"), ('r', "6"), ('s', "*"), ('t', "1"),
    ('u', "5"), ('v', "t"), ('w', "4"), ('x', "s"), ('y', "r"),
    ('z', "q"), ('.', "."), ('!', "!"), ('?', "?"), (',', ","),
    (';', ";"), (' ', " "),
];

pub fn to_cipher(input: &str) -> String {
    let cipher_map: HashMap<char, &str> = CIPHER.iter().copied().collect();
    let cleaned = deunicode(input);
    
    cleaned
        .to_lowercase()
        .chars()
        .filter_map(|c| cipher_map.get(&c).copied())
        .collect::<String>()
}

pub fn from_cipher(input: &str) -> String {
    let cipher_map: HashMap<&str, char> = CIPHER.iter()
        .map(|(latin, cipher)| (*cipher, *latin))
        .collect();
    
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == ' ' {
            result.push(' ');
            continue;
        }
        
        let mut cipher_char = c.to_string();
        
        while let Some(&next_c) = chars.peek() {
            if !next_c.is_whitespace() && !cipher_map.contains_key(cipher_char.as_str()) {
                cipher_char.push(next_c);
                chars.next();
            } else {
                break;
            }
        }
        
        if let Some(&latin_char) = cipher_map.get(cipher_char.as_str()) {
            result.push(latin_char);
        } else {
            result.push_str(&cipher_char);
        }
    }
    
    result
}

// Testing
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_from_cipher() {
        let original = "hello world";
        let encrypted = to_cipher(original);
        let decrypted = from_cipher(&encrypted);
        assert_eq!(decrypted, original);
    }
    
    #[test]
    fn test_direct_conversion() {
        // hello world = x#ww7 476wd
        assert_eq!(from_cipher("x#ww7 476wd"), "hello world");
        // hello! = x#ww7!
        assert_eq!(from_cipher("x#ww7!"), "hello!");
    }
    
    #[test]
    fn test_encryption_example() {
        assert_eq!(to_cipher("hello"), "x#ww7");
        assert_eq!(to_cipher("world"), "476wd");
        assert_eq!(to_cipher("test"), "1#*1");
    }
}