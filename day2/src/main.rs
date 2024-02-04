use std::fs;

fn main() {
    let contents = fs::read_to_string("./2.txt")
        .expect("Unable to read file, check whether the file is in appropriate path.");

    let split_data = contents.trim().split("\n").enumerate();

    let mut s1 = 0;
    let mut s2 = 0;
    for (_, m) in split_data {
        // println!("{:?}", m);
        s1 += score1(m);
        s2 += score2(m);
    }

    println!("{:?}, {:?}", s1, s2);
}

#[derive(Debug)]
enum Game {
    Rock,
    Paper,
    Scissor,
}

impl Game {
    fn play(&self, g: Game) -> u16 {
        match (self, g) {
            (Game::Rock, Game::Rock) => 3,
            (Game::Rock, Game::Paper) => 0,
            (Game::Rock, Game::Scissor) => 6,
            (Game::Paper, Game::Rock) => 6,
            (Game::Paper, Game::Paper) => 3,
            (Game::Paper, Game::Scissor) => 0,
            (Game::Scissor, Game::Rock) => 0,
            (Game::Scissor, Game::Paper) => 6,
            (Game::Scissor, Game::Scissor) => 3,
        }
    }
}

fn score1(m: &str) -> u16 {
    let mut m = m.trim().split(" ").into_iter();
    let p1 = match m.next() {
        Some(s) => match s {
            "A" => Game::Rock,
            "B" => Game::Paper,
            "C" => Game::Scissor,
            _ => panic!("You fucked up!"),
        },
        None => panic!("You fucked up!"),
    };
    let p2 = match m.next() {
        Some(s) => match s {
            "X" => Game::Rock,
            "Y" => Game::Paper,
            "Z" => Game::Scissor,
            _ => panic!("You fucked up!"),
        },
        None => panic!("You fucked up!"),
    };

    // println!("P1 played {:?}, P2 played {:?}", p1, p2);

    let s = p2.play(p1)
        + match p2 {
            Game::Rock => 1,
            Game::Paper => 2,
            Game::Scissor => 3,
        };

    // println!("Score -> {:?}", s);
    return s;
}

fn score2(m: &str) -> u16 {
    let mut m = m.trim().split(" ").into_iter();
    let p1 = match m.next() {
        Some(s) => match s {
            "A" => Game::Rock,
            "B" => Game::Paper,
            "C" => Game::Scissor,
            _ => panic!("You fucked up!"),
        },
        None => panic!("You fucked up!"),
    };

    let p2 = match m.next() {
        Some(s) => match s {
            "X" => match p1 {
                Game::Rock => Game::Scissor,
                Game::Paper => Game::Rock,
                Game::Scissor => Game::Paper,
            },
            "Y" => match p1 {
                Game::Rock => Game::Rock,
                Game::Paper => Game::Paper,
                Game::Scissor => Game::Scissor,
            },
            "Z" => match p1 {
                Game::Rock => Game::Paper,
                Game::Paper => Game::Scissor,
                Game::Scissor => Game::Rock,
            },
            _ => panic!("You fucked up!"),
        },
        None => panic!("You fucked up!"),
    };

    let s = p2.play(p1)
        + match p2 {
            Game::Rock => 1,
            Game::Paper => 2,
            Game::Scissor => 3,
        };

    // println!("Score -> {:?}", s);
    return s;
}
