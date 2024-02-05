use std::fs;

fn main() {
    let contents = fs::read_to_string("./4.txt")
        .expect("Unable to read file, make sure file is in appropriate path.");

    let split_data: Vec<&str> = contents.trim().split("\n").collect();

    let mut ans = 0;
    let mut ans2 = 0;
    for i in split_data {
        if covers(i) {
            ans += 1;
        }
        if covers2(i) {
            ans2 += 1;
        }
    }
    println!("{:?} {:?}", ans, ans2);
}

struct Interval {
    lo: u8,
    hi: u8,
}

impl Interval {
    fn contains(&self, i: &Interval) -> bool {
        return self.lo <= i.lo && self.hi >= i.hi;
    }

    fn overlaps(&self, i: &Interval) -> bool {
        return self.lo <= i.hi && i.lo <= self.hi;
    }
}

fn covers2(s: &str) -> bool {
    let mut m = s.split(",").into_iter();

    let i1 = match m.next() {
        Some(s) => {
            let v: Vec<&str> = s.split("-").collect();
            Interval {
                lo: v[0].parse().expect("Not a u8"),
                hi: v[1].parse().expect("Not a u8"),
            }
        }
        None => panic!("You shouldn't be here"),
    };

    let i2 = match m.next() {
        Some(s) => {
            let v: Vec<&str> = s.split("-").collect();
            Interval {
                lo: v[0].parse().expect("Not a u8"),
                hi: v[1].parse().expect("Not a u8"),
            }
        }
        None => panic!("You shouldn't be here"),
    };

    return i1.overlaps(&i2) || i2.overlaps(&i1);
}

fn covers(s: &str) -> bool {
    let mut m = s.split(",").into_iter();

    let i1 = match m.next() {
        Some(s) => {
            let v: Vec<&str> = s.split("-").collect();
            Interval {
                lo: v[0].parse().expect("Not a u8"),
                hi: v[1].parse().expect("Not a u8"),
            }
        }
        None => panic!("You shouldn't be here"),
    };

    let i2 = match m.next() {
        Some(s) => {
            let v: Vec<&str> = s.split("-").collect();
            Interval {
                lo: v[0].parse().expect("Not a u8"),
                hi: v[1].parse().expect("Not a u8"),
            }
        }
        None => panic!("You shouldn't be here"),
    };

    return i1.contains(&i2) || i2.contains(&i1);
}
