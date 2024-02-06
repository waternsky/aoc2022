use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("./5.txt")
        .expect("Unable to read file, make sure it's in appropriate path.");

    let split_data: Vec<&str> = contents.split("\n\n").collect();

    let arrangement = split_data[0];
    let arrangement1 = parse(arrangement);
    let arrangement2 = arrangement1.clone();

    let moves: Vec<[usize; 3]> = split_data[1].trim().split("\n").into_iter().map(|s| f(s)).collect();
   
    println!("{:?}", cargo_crane_9000(arrangement1, &moves));
    println!("{:?}", cargo_crane_9001(arrangement2, &moves));
}

fn cargo_crane_9001(mut arrangement: HashMap<usize, Vec<char>>, moves: &Vec<[usize; 3]>) -> String {
    for m in moves {
        let from = m[1];
        let to = m[2];
        let amt = m[0];
        
        let v = arrangement.get_mut(&from).unwrap();
        let tmp: Vec<char> = v.drain(v.len()-amt..v.len()).collect();
        arrangement.get_mut(&to).unwrap().extend(tmp);
    } 

    let ans: Vec<(usize, char)> = arrangement.into_iter().map(|(k, v)| g(k, v)).collect();

    let mut s = String::new();
    for i in 1..=ans.len() {
        let mut ch = ' ';
        for a in &ans {
            if a.0 == i {
                ch = a.1;
                break;
            }
        }
        s.push(ch);
    }

    return s;
}

fn cargo_crane_9000(mut arrangement: HashMap<usize, Vec<char>>, moves: &Vec<[usize; 3]>) -> String {
    for m in moves {
        let from = m[1];
        let to = m[2];
        let amt = m[0];
        for _ in 0..amt {
            let tmp = arrangement.get_mut(&from).unwrap().pop().unwrap();
            arrangement.get_mut(&to).unwrap().push(tmp);
        }
    } 

    let ans: Vec<(usize, char)> = arrangement.into_iter().map(|(k, v)| g(k, v)).collect();

    let mut s = String::new();
    for i in 1..=ans.len() {
        let mut ch = ' ';
        for a in &ans {
            if a.0 == i {
                ch = a.1;
                break;
            }
        }
        s.push(ch);
    }

    return s;
}

fn g(k: usize, v: Vec<char>) -> (usize, char) {
    (k, v[v.len()-1])
}

fn f(s: &str) -> [usize; 3] {
    let t: Vec<&str> = s.split(" ").collect();
    let u1: usize = t[1].parse().expect("Not a number");
    let u2: usize = t[3].parse().expect("Not a number");
    let u3: usize = t[5].parse().expect("Not a number");

    return [u1, u2, u3];
}

fn parse(s: &str) -> HashMap<usize, Vec<char>> {
    let split_data: Vec<&str> = s.split("\n").collect();
    let mut ans: HashMap<usize, Vec<char>> = HashMap::new();
    for i in 0..split_data.len() - 1 {
        let row: Vec<char> = split_data[i].chars().collect();
        for j in 0..=row.len()/4 {
            if row[4*j] != ' ' {
                match ans.get_mut(&(j+1)) {
                    Some(v) => v.push(row[4*j+1]),
                    None => {
                        ans.insert(j+1, vec![row[4*j+1]]);
                    },
                };
            }
        }
    }

    for (_, v) in ans.iter_mut() {
        v.reverse();
    }
    return ans;
}
