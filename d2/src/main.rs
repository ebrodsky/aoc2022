use std::fs;
#[derive(PartialEq, Clone)]
enum Choice{
    Rock,
    Paper,
    Scissors,
}

impl Choice{
    fn choice_value(&self) -> i32 {
        match self{
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3
        }
    }

    fn outcome(ch1: &Choice, ch2: &Choice) -> (i32, i32){
        if ch1 == ch2{
            return (ch1.choice_value() + 3, ch2.choice_value() + 3);
        }
        else{
            match (ch1, ch2){
                (Choice::Rock, Choice::Scissors) | 
                (Choice::Paper, Choice::Rock) | 
                (Choice::Scissors, Choice::Paper) => return (ch1.choice_value() + 6, ch2.choice_value()),
                _ => return (ch1.choice_value(), ch2.choice_value() + 6)
            }
        }
    }
}

fn get_choice(letter: &str) -> Choice{
    match letter{
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissors,
        _ => unreachable!()
    }
}

fn desired_outcome(choice: &Choice, letter: &str) -> Choice{
    match letter{
        "Y" => choice.clone(),
        "Z" => match choice{
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        },
        "X" => match choice{
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        },
        _ => unreachable!()
    }
}

fn main() {
    let file_path = "/home/ebrodsky/aoc2022/d2/input.txt";
    let res1 = part_one(file_path);
    let res2 = part_two(file_path);
    println!("{}", res1);
    println!("{}", res2);
}

fn part_one(file_path: &str) -> i32{
    /*
    A, X -> Rock, 1
    B, Y -> Paper, 2
    C, Z -> Scissors, 3
    Lose -> 0
    Draw -> 3
    Win -> 6
    */
    let f = fs::read_to_string(file_path).expect("file not found");
    let games:Vec<&str> = f.split("\n").collect();
    let mut scores = (0, 0);
    for game in &games{
        let mut iter = game.split(" ");
        let (p1, p2) = (iter.next().unwrap(), iter.next().unwrap());
        let opponent = get_choice(p1);
        let me = get_choice(p2);
        let outcome = Choice::outcome(&opponent, &me);
        scores.0 += outcome.0;
        scores.1 += outcome.1;
    }
    return scores.1;
}

fn part_two(file_path: &str) -> i32{
    /*
    X -> lose
    Y -> Draw
    Z -> Win
    */

    let f = fs::read_to_string(file_path).expect("file not found");
    let games:Vec<&str> = f.split("\n").collect();
    let mut scores = (0, 0);
    for game in &games{
        let mut iter = game.split(" ");
        let (p1, p2) = (iter.next().unwrap(), iter.next().unwrap());
        let opponent = get_choice(p1);
        let me = desired_outcome(&opponent, p2);
        let outcome = Choice::outcome(&opponent, &me);
        scores.0 += outcome.0;
        scores.1 += outcome.1;
    }
    return scores.1;
}