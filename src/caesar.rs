use deunicode::deunicode;

pub fn caesar_encrypt(input: &str, shift: i32) -> String {
    let cleaned = deunicode(input);
    
    cleaned
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shifted = (((c as u8 - base) as i32 + shift).rem_euclid(26)) as u8;
                (base + shifted) as char
            } else {
                c // Keep non-alphabetic characters unchanged
            }
        })
        .collect()
}

pub fn caesar_decrypt(input: &str, shift: i32) -> String {
    caesar_encrypt(input, -shift)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_encrypt() {
        assert_eq!(caesar_encrypt("hello", 3), "khoor");
        assert_eq!(caesar_encrypt("HELLO", 3), "KHOOR");
        assert_eq!(caesar_encrypt("Hello World!", 3), "Khoor Zruog!");
    }

    #[test]
    fn test_caesar_decrypt() {
        assert_eq!(caesar_decrypt("khoor", 3), "hello");
        assert_eq!(caesar_decrypt("KHOOR", 3), "HELLO");
        assert_eq!(caesar_decrypt("Khoor Zruog!", 3), "Hello World!");
    }

    #[test]
    fn test_caesar_roundtrip() {
        let original = "Hello World!";
        let encrypted = caesar_encrypt(original, 13);
        let decrypted = caesar_decrypt(&encrypted, 13);
        assert_eq!(decrypted, original);
    }

    #[test]
    fn test_caesar_wrap_around() {
        assert_eq!(caesar_encrypt("xyz", 3), "abc");
        assert_eq!(caesar_encrypt("XYZ", 3), "ABC");
        assert_eq!(caesar_decrypt("abc", 3), "xyz");
    }

    #[test]
    fn test_caesar_negative_shift() {
        assert_eq!(caesar_encrypt("hello", -3), "ebiil");
        assert_eq!(caesar_decrypt("ebiil", -3), "hello");
    }
}