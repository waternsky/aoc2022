use std::fs;

fn main() {
    let contents = fs::read_to_string("./1.txt")
        .expect("Unable to read file, check whether file is in appropriate path.");

    let split_data = contents.trim().split("\n\n").enumerate();

    let mut max: u32 = 0;
    let mut ans: Vec<u32> = vec![];
    for (_, item) in split_data {
        let result = sum(item);
        ans.push(result);
        if result > max {
            max = result;
        }
    }

    let s = sort(&mut ans);
    println!("{:?} and {:?}", max, s[0] + s[1] + s[2]);
}

// bubble sort
fn sort(v: &mut Vec<u32>) -> &Vec<u32> {
    for i in 0..v.len() {
        for j in 0..v.len() - i - 1 {
            if v[j + 1] > v[j] {
                let tmp = v[j];
                v[j] = v[j + 1];
                v[j + 1] = tmp;
            }
        }
    }

    return v;
}

fn sum(s: &str) -> u32 {
    let nums = s.split("\n").enumerate();

    let mut sum: u32 = 0;
    for (_, num) in nums {
        let num: u32 = num.trim().parse().expect("Unable to convert to number");
        sum = sum + num;
    }

    return sum;
}
