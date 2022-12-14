use std::fs;

fn main() {
    let path = "/home/ebrodsky/aoc2022/d5/input.txt";
    let res1 = part_one(path);
    let res2 = part_two(path);
    println!("{:#?}", res1);
    println!("{:#?}", res2);
}
fn part_one(file_path: &str) -> String{
    let s1 = "D H R Z S P W Q".chars().rev().collect::<String>();
    let s2 = "F H Q W R B V".chars().rev().collect::<String>();
    let s3 = "H S V C".chars().rev().collect::<String>();
    let s4 = "G F H".chars().rev().collect::<String>();
    let s5 = "Z B J G P".chars().rev().collect::<String>();
    let s6 = "L F W H J T Q".chars().rev().collect::<String>();
    let s7 = "N J V L D W T Z".chars().rev().collect::<String>();
    let s8 = "F H G J C Z T D".chars().rev().collect::<String>();
    let s9 = "H B M V P W".chars().rev().collect::<String>();

    let stacks = Vec::from_iter([s1.as_str(), s2.as_str(), s3.as_str(), s4.as_str(), s5.as_str(), s6.as_str(), s7.as_str(), s8.as_str(), s9.as_str()]);
    let mut stacks: Vec<Vec<&str>> = stacks.iter().map(|s| s.split(" ").collect()).collect();

    let f = fs::read_to_string(file_path).expect("file not found");
    let mut iter = f.split("\n");

    for _ in 0..10{
        iter.next();
    }
    //After this, calling iter.next() will give the first line of the moves

    while let Some(action) = iter.next(){
        //1: number of rates, 3: from, 5: to
        let nums:Vec<&str> = action.split(" ").collect();
        let num_crates = nums.get(1).unwrap().parse::<i32>().unwrap();
        let source_crate = nums.get(3).unwrap().parse::<usize>().unwrap()-1;
        let dest_crate = nums.get(5).unwrap().parse::<usize>().unwrap()-1;

        for _ in 0..num_crates{
            let item = stacks.get_mut(source_crate).unwrap().pop().unwrap();
            stacks.get_mut(dest_crate).unwrap().push(item);
        }
    }

    let result = stacks.into_iter().map(|v| v.last().unwrap().to_string()).collect::<String>();
    return result;
}

fn part_two(file_path: &str) -> String{
    let s1 = "D H R Z S P W Q".chars().rev().collect::<String>();
    let s2 = "F H Q W R B V".chars().rev().collect::<String>();
    let s3 = "H S V C".chars().rev().collect::<String>();
    let s4 = "G F H".chars().rev().collect::<String>();
    let s5 = "Z B J G P".chars().rev().collect::<String>();
    let s6 = "L F W H J T Q".chars().rev().collect::<String>();
    let s7 = "N J V L D W T Z".chars().rev().collect::<String>();
    let s8 = "F H G J C Z T D".chars().rev().collect::<String>();
    let s9 = "H B M V P W".chars().rev().collect::<String>();

    let stacks = Vec::from_iter([s1.as_str(), s2.as_str(), s3.as_str(), s4.as_str(), s5.as_str(), s6.as_str(), s7.as_str(), s8.as_str(), s9.as_str()]);
    let mut stacks: Vec<Vec<&str>> = stacks.iter().map(|s| s.split(" ").collect()).collect();

    let f = fs::read_to_string(file_path).expect("file not found");
    let mut iter = f.split("\n");

    for _ in 0..10{
        iter.next();
    }
    //After this, calling iter.next() will give the first line of the moves

    while let Some(action) = iter.next(){
        //1: number of rates, 3: from, 5: to
        let nums:Vec<&str> = action.split(" ").collect();
        let num_crates = nums.get(1).unwrap().parse::<i32>().unwrap();
        let source_crate = nums.get(3).unwrap().parse::<usize>().unwrap()-1;
        let dest_crate = nums.get(5).unwrap().parse::<usize>().unwrap()-1;
        let mut to_move:Vec<&str> = Vec::new();
        for _ in 0..num_crates{
            to_move.push(stacks.get_mut(source_crate).unwrap().pop().unwrap());
        }

        let mut to_move = to_move.iter().rev();
        for _ in 0..num_crates{
            stacks.get_mut(dest_crate).unwrap().push(to_move.next().unwrap());
        }

    }

    let result = stacks.into_iter().map(|v| v.last().unwrap().to_string()).collect::<String>();
    return result;
}

