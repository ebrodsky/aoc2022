use std::fs;
use std::collections::HashMap;

struct Node{
    name: String,
    size: Option<i32>,
    parent: Option<Box<Node>>,
    children: Option<Vec<Box<Node>>>,
}

impl Node{
    fn new(name: String) -> Self{
        return Self { name, size: None, parent: None, children: None }
    }

    fn is_dir(&self) -> bool{
        if self.children.is_some(){
            return true
        }
        return false;
    }

    fn add_children(&mut self, node_names: Vec<&str>){
        if let Some(children) = &mut self.children{
            let mut names = node_names.iter();
            while let Some(name) = names.next(){
                if !children.iter().map(|node|&node.name).collect::<Vec<&String>>().contains(&&name.to_string()){
                    children.push(Box::new(Node::new(name.to_string())));
                }
            }
        }
    }
}

fn main() {
    let path = "/home/ebrodsky/aoc2022/d7/input.txt";
}

fn part_one(file_path: &str) -> i32{
    let f = fs::read_to_string(file_path).expect("file not found");
    let sum = 0;
    let lines = f.split("\n");
    let mut cur_node = &mut Box::new(Node::new("".to_string()));
    for line in lines{
        let mut args = line.split(" ");
        match args.next().unwrap(){
            "$" => { //This is a command
                match args.next().unwrap(){
                    "cd" => {
                        let node_name = args.next().unwrap();
                        cur_node.add_children(vec![node_name]);
                        let children = cur_node.children.as_ref().unwrap();
                        cur_node = children.iter().find(|node| &node.name == node_name).as_mut().unwrap();
                    },
                    "ls" => {
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
    return sum;
}