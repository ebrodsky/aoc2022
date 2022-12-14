use std::fs;

fn main() {
    let file_path:&str = "/home/ebrodsky/aoc2022/d1/input.txt";
    let res1 = part_one(file_path);
    let res2 = part_two(file_path);
    println!("{}", res1);
    println!("{}", res2);
}

fn part_one(file_path: &str) -> i32 {
    let f = fs::read_to_string(file_path).expect("file not found");
    let contents:Vec<&str> = f.split("\n\n").collect();
    let mut max: i32 = 0;
    for item in &contents{
        let calories: i32= item.split("\n").map(|s| s.parse::<i32>().unwrap()).sum();
        if max < calories{
            max = calories;
        }
    }

    return max;
}

fn part_two(file_path: &str) -> i32 {
    let f = fs::read_to_string(file_path).expect("file not found");
    let mut top_three = [0;3];
    let mut cur_min_idx = 0;
    let contents:Vec<&str> = f.split("\n\n").collect();
    for item in &contents{
        let calories: i32= item.split("\n").map(|s| s.parse::<i32>().unwrap()).sum();
        if calories > top_three[cur_min_idx]{
            top_three[cur_min_idx] = calories;
            cur_min_idx = top_three
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index).unwrap();
            println!("{}", cur_min_idx);
            println!("{:#?}", top_three);
        }
    }

    return top_three.iter().sum();
}