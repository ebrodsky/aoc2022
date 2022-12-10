use std::fs;

fn main() {
    let path = "/home/ebrodsky/aoc2022/d4/input.txt";
    let res1 = part_one(path);
    let res2 = part_two(path);
    println!("{}", res1);
    println!("{}", res2);
}

fn part_one(file_path: &str) -> i32 {
    let mut count = 0;
    let f = fs::read_to_string(file_path).expect("file not found");
    let pairs = f.split("\n");

    for pair in pairs{
        let mut pairs = pair.split(",");
        let range1 = pairs.next().unwrap();
        let range2 = pairs.next().unwrap();
        let mut range1 = range1.split("-");
        let mut range2 = range2.split("-");
        let (min1, max1) = (range1.next().unwrap().parse::<i32>().unwrap(), 
                                        range1.next().unwrap().parse::<i32>().unwrap());
        let (min2, max2) = (range2.next().unwrap().parse::<i32>().unwrap(), 
                                        range2.next().unwrap().parse::<i32>().unwrap());

        if (min2 >= min1 && max2 <= max1) || (min1 >= min2 && max1 <= max2){
            count += 1;
        }
    }

    return count;
}

fn part_two(file_path: &str) -> i32{
    let mut count = 0;
    let f = fs::read_to_string(file_path).expect("file not found");
    let pairs = f.split("\n");

    for pair in pairs{
        let mut pairs = pair.split(",");
        let range1 = pairs.next().unwrap();
        let range2 = pairs.next().unwrap();
        let mut range1 = range1.split("-");
        let mut range2 = range2.split("-");
        let (min1, max1) = (range1.next().unwrap().parse::<i32>().unwrap(), 
                                        range1.next().unwrap().parse::<i32>().unwrap());
        let (min2, max2) = (range2.next().unwrap().parse::<i32>().unwrap(), 
                                        range2.next().unwrap().parse::<i32>().unwrap());

        if (min1 <= min2 && min2 <= max1) || (min1 <= max2 && max2 <= max1)
        || (min2 <= min1 && min1 <= max2) || (min2 <= max1 && max1 <= max2){
            count += 1;
        }
    }

    return count;
}
