use std::{collections::HashMap, fs, usize};

fn main() {
    let contents = fs::read_to_string("./6.txt")
        .expect("Unable to read file, make sure file is in appropriate path");

    let s = contents.trim();

    println!("Part 1 ans = {:?}", marker(s, 4));
    println!("Part 2 ans = {:?}", marker(s, 14));
}

fn marker(s: &str, distinct_chars: usize) -> usize {
    let mut start = 0;
    let arr: Vec<char> = s.chars().collect();
    while (start + distinct_chars < s.len())
        && !distinct(arr[start..start + distinct_chars].to_vec())
    {
        start += 1;
    }
    if start + distinct_chars >= s.len() {
        panic!("No marker in string");
    }
    return start + distinct_chars;
}

fn distinct(v: Vec<char>) -> bool {
    let mut h: HashMap<char, usize> = HashMap::new();
    for ch in v {
        h.entry(ch).and_modify(|e| *e += 1).or_insert(1);
    }
    for (_, v) in h {
        if v > 1 {
            return false;
        }
    }
    return true;
}
