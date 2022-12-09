use std::fs;
use std::collections::HashSet;
fn main() {
    let file_path: &str = "/home/ebrodsky/aoc2022/d3/input.txt";
    let res1 = part_one(file_path);
    let res2 = part_two(file_path);
    println!("{}", res1);
    println!("{}", res2);
}


fn part_one(file_path: &str) -> usize{
    let f = fs::read_to_string(file_path).expect("file not found");
    let mut alpha_cap = (b'A'..=b'Z').map(|c| c as char).filter(|c| c.is_alphabetic())
    .collect::<Vec<char>>();
    let mut alphabet = (b'a'..=b'z').map(|c| c as char).filter(|c| c.is_alphabetic())
    .collect::<Vec<char>>();
    alphabet.append(&mut alpha_cap);
    let rucksacks:Vec<&str> = f.split("\n").collect();
    let mut priorities: usize = 0;
    for sack in &rucksacks{
        let num_items = (*sack).len();
        let first_half = &sack[0..num_items/2];
        let second_half = &sack[num_items/2..];

        let set1:HashSet<char>= HashSet::from_iter(first_half.chars().collect::<Vec<char>>());
        let set2:HashSet<char> = HashSet::from_iter(second_half.chars().collect::<Vec<char>>());

        let mut intersection = set1.intersection(&set2);
        let x = intersection.next().unwrap();
        priorities += alphabet.iter().position(|&t|t == *x).unwrap() + 1;
    }
    return priorities;
}

fn part_two(file_path: &str) -> usize{
    let f = fs::read_to_string(file_path).expect("file not found");
    let mut alpha_cap = (b'A'..=b'Z').map(|c| c as char).filter(|c| c.is_alphabetic())
    .collect::<Vec<char>>();
    let mut alphabet = (b'a'..=b'z').map(|c| c as char).filter(|c| c.is_alphabetic())
    .collect::<Vec<char>>();
    alphabet.append(&mut alpha_cap);
    println!("{:#?}", alphabet);
    let mut groups:Vec<[&str; 3]> = Vec::new();
    let mut iter = f.split("\n");
    let mut priorities = 0;

    while let Some(x) = iter.next(){
        groups.push([x, iter.next().unwrap(), iter.next().unwrap()])
    }

    for group in &groups{
        let set1:HashSet<char> = HashSet::from_iter(group[0].chars().collect::<Vec<char>>());
        let set2:HashSet<char> = HashSet::from_iter(group[1].chars().collect::<Vec<char>>());
        let set3:HashSet<char> = HashSet::from_iter(group[2].chars().collect::<Vec<char>>());
        let intersection:HashSet<char> = set1.intersection(&set2).cloned().collect();
        //.cloned() is basically syntax sugar for .map(|x| x.clone())
        let mut intersection = intersection.intersection(&set3).cloned();
        let x = intersection.next().unwrap();
        priorities += alphabet.iter().position(|&t|t == x).unwrap() + 1;
    }
    return priorities;
}