use std::collections::HashMap;
use deunicode::deunicode;

pub fn to_morse(input: &str) -> String {
    let morse = get_morse_map();
    let cleaned = deunicode(input); // č → c, ř → r, etc.

    cleaned
        .to_lowercase()
        .split_whitespace()
        .map(|word| {
            word.chars()
                .filter_map(|c| morse.get(&c))
                .cloned()
                .collect::<Vec<_>>()
                .join("/")
        })
        .collect::<Vec<_>>()
        .join("//")
}

pub fn from_morse(morse_code: &str) -> String {
    let morse = get_morse_map();
    let reverse_morse: HashMap<_, _> = morse.into_iter().map(|(k, v)| (v, k)).collect();

    morse_code
        .split("//")
        .map(|word| {
            word.split('/')
                .filter_map(|code| reverse_morse.get(code))
                .cloned()
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn get_morse_map() -> HashMap<char, &'static str> {
    [
        ('a', ".-"),    ('b', "-..."),  ('c', "-.-."),  ('d', "-.."),   ('e', "."),
        ('f', "..-."),  ('g', "--."),   ('h', "...."),  ('i', ".."),    ('j', ".---"),
        ('k', "-.-"),   ('l', ".-.."),  ('m', "--"),    ('n', "-."),    ('o', "---"),
        ('p', ".--."),  ('q', "--.-"),  ('r', ".-."),   ('s', "..."),   ('t', "-"),
        ('u', "..-"),   ('v', "...-"),  ('w', ".--"),   ('x', "-..-"),  ('y', "-.--"),
        ('z', "--.."),  ('0', "-----"), ('1', ".----"), ('2', "..---"), ('3', "...--"),
        ('4', "....-"), ('5', "....."), ('6', "-...."), ('7', "--..."), ('8', "---.."),
        ('9', "----."), 
        ('.', ".-.-.-"), (',', "--..--"), ('?', "..--.."), ('!', "-.-.--"), 
        ('-', "-....-"), ('@', ".--.-."), ('(', "-.--."), (')', "-.--.-"),
        (':', "---..."), (';', "-.-.-."), ('=', "-...-"), ('+', ".-.-."), ('_', "..--.-"),
        ('\'', ".----."), ('&', ".-..."),
    ]
    .iter()
    .cloned()
    .collect()
}
