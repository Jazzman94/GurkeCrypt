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
