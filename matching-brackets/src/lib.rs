use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let bracket_map: HashMap<char, char> = [('(', ')'), ('{', '}'), ('[', ']')]
        .iter()
        .cloned()
        .collect();

    let mut stack: Vec<char> = Vec::new();

    for ch in string.chars() {
        if bracket_map.contains_key(&ch) {
            stack.push(ch);
        } else if bracket_map.values().any(|&val| val == ch) {
            if let Some(key) = stack.pop() {
                if bracket_map[&key] != ch {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    stack.is_empty()
}
