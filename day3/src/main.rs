use std::fs;

fn main() {
    let contents = fs::read_to_string("./3.txt")
        .expect("Unable to read file, check whether the file is in appropriate path.");

    let split_data = contents.trim().split("\n").enumerate();

    let mut sum = 0;
    for (_, rucksack) in split_data {
        match common_item(rucksack) {
            Some(c) => match priorities(c) {
                Some(priority) => {
                    sum = sum + priority;
                }
                None => panic!("You fucked up!"),
            },
            None => panic!("You fucked up!"),
        }
    }
    println!("{:?}", sum);

    let mut sum2 = 0;
    let v: Vec<&str> = contents.trim().split("\n").collect();
    for i in 0..v.len() / 3 {
        match common_item2(v[3 * i], v[3 * i + 1], v[3 * i + 2]) {
            Some(c) => match priorities(c) {
                Some(priority) => {
                    sum2 = sum2 + priority;
                }
                None => panic!("You fucked up!"),
            },
            None => panic!("You fucked up!"),
        }
    }
    println!("{:?}", sum2);
}

fn common_item2(a: &str, b: &str, c: &str) -> Option<char> {
    for c1 in a.chars() {
        for c2 in b.chars() {
            for c3 in c.chars() {
                if c1 == c2 && c1 == c3 {
                    return Some(c1);
                }
            }
        }
    }
    return None;
}

fn priorities(c: char) -> Option<u32> {
    if c >= 'a' && c <= 'z' {
        return Some(c as u32 - 'a' as u32 + 1);
    }
    if c >= 'A' && c <= 'Z' {
        return Some(c as u32 - 'A' as u32 + 27);
    }
    return None;
}

fn common_item(s: &str) -> Option<char> {
    for ch1 in s[..s.len() / 2].chars() {
        for ch2 in s[s.len() / 2..].chars() {
            if ch1 == ch2 {
                return Some(ch1);
            }
        }
    }
    return None;
}
