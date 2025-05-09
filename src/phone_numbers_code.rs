use std::collections::HashMap;
use deunicode::deunicode;

pub fn to_phone_number(input: &str) -> String {
    let phone_map = [
        ('a', "2"),    ('b', "22"),  ('c', "222"),  ('d', "3"),   ('e', "33"),
        ('f', "333"),  ('g', "4"),   ('h', "44"),  ('i', "444"),    ('j', "5"),
        ('k', "55"),   ('l', "555"),  ('m', "6"),    ('n', "66"),    ('o', "666"),
        ('p', "7"),  ('q', "77"),  ('r', "777"),   ('s', "7777"),   ('t', "8"),
        ('u', "88"),   ('v', "888"),  ('w', "9"),   ('x', "99"),  ('y', "999"),
        ('z', "9999"), ('.',"."), ('!',"!"), ('?',"?"), (',',","), (';',";"),
    ].iter().cloned().collect::<HashMap<_, _>>();

    let cleaned: String = deunicode(input); // č → c, ř → r, ů → u, etc
    
    cleaned
        .to_lowercase()
        .split_whitespace()
        .map(|word| {
            word.chars()
                .filter_map(|c| phone_map.get(&c))
                .cloned()
                .collect::<Vec<_>>()
                .join("/")
        })
        .collect::<Vec<_>>()
        .join("//")
}

pub fn from_phone_number(input: &str) -> String {
    let phone_map = [
        ("2", "a"),    ("22", "b"),  ("222", "c"),  ("3", "d"),   ("33", "e"),
        ("333", "f"),  ("4", "g"),   ("44", "h"),  ("444", "i"),    ("5", "j"),
        ("55", "k"),   ("555", "l"),  ("6", "m"),    ("66", "n"),    ("666", "o"),
        ("7", "p"),  ("77", "q"),  ("777", "r"),   ("7777", "s"),   ("8", "t"),
        ("88", "u"),   ("888", "v"),  ("9", "w"),   ("99", "x"),  ("999", "y"),
        ("9999","z")
    ].iter().cloned().collect::<HashMap<_, _>>();

    let cleaned: String = deunicode(input); // č → c, ř → r, ů → u, etc

    cleaned
        .split("//")
        .map(|word| {
            word.split('/')
                .filter_map(|num| phone_map.get(num))
                .cloned()
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}