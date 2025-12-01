//use std::env;
use std::fs;

fn part_a(input: &String) -> i32 {
    let vector: Vec<i32> = parse_input(input);

    let mut increased_counter = 0;
    for (index, item) in vector.iter().enumerate() {
        if index != 0 && item > &vector[index - 1] {
            increased_counter += 1;
        }
    }
    return increased_counter;
}

fn part_b(input: &String) -> i32 {
    let vector: Vec<i32> = parse_input(input);

    let mut increased_counter = 0;
    for (index, item) in vector.iter().enumerate() {
        if index >= 3 && item > &vector[index - 3] {
            increased_counter += 1;
        }
    }
    return increased_counter;
}

fn part_c(input: &String, window: usize) -> i32 {
    let vector: Vec<i32> = parse_input(input);

    let mut increased_counter = 0;
    for (index, item) in vector.iter().enumerate() {
        if index >= window && item > &vector[index - window] {
            increased_counter += 1;
        }
    }
    return increased_counter;
}

fn parse_input(input: &String) -> Vec<i32> {
    let mut vector: Vec<i32> = Vec::new();
    
    for item in input.split_whitespace() {
        vector.push(item.parse::<i32>().unwrap());
    }

    return vector;
}

fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents: String = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    //println!("Our current string: {:?}", contents);

    println!("Part 1: {:?}", part_a(&contents));
    println!("Part 2: {:?}", part_b(&contents));
    println!("Part 1 with window: {:?}", part_c(&contents, 1));
    println!("Part 2 with window: {:?}", part_c(&contents, 3));
}
