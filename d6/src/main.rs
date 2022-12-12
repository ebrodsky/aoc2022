use std::{fs, error::Error};
use std::collections::HashSet;
fn main() {
    let path = "/home/ebrodsky/aoc2022/d6/input.txt";
    let res1 = part_one(path);
    let res2 = part_two(path);
    println!("{:#?}", res1);
    println!("{:#?}", res2);
}

fn part_one(file_path: &str) -> Result<usize, &str>{
    let f = fs::read_to_string(file_path).expect("file not found");
    let stream:Vec<char> = f.chars().collect();
    println!("{:#?}", stream);

    for mut i in 0..stream.len()-4{
        let x1 = stream.get(i+0).unwrap();
        let x2 = stream.get(i+1).unwrap();
        let x3 = stream.get(i+2).unwrap();
        let x4 = stream.get(i+3).unwrap();
        println!("{x1}, {x2}, {x3}, {x4}");

        if *x1 != *x2 && *x1 != *x3 && *x1 != *x4 &&
            *x2 != *x3 && *x2 != *x4 &&
            *x3 != *x4{
                return Ok(i+4);
            }
    }
    return Err("Could not find start");
}

fn part_two(file_path: &str) -> Result<usize, &str>{

    let f = fs::read_to_string(file_path).expect("file not found");
    let stream:Vec<char> = f.chars().collect();
    println!("{:#?}", stream);

    for mut i in 0..stream.len()-14{
        let slice = &stream[i..i+14];
        let set:HashSet<&char> = HashSet::from_iter(slice.iter());
        if slice.len() == set.len(){
            return Ok(i+14);
        }
    }
    return Err("Could not find start");
}