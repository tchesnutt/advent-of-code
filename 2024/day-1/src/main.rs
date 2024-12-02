use std::fs::read_to_string;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::collections::HashMap;


fn main() {
    let lines = read_lines("./input.txt");

    
    calc_distance_between(lines.clone());
    calc_similarity_score(lines.clone());
}

fn calc_similarity_score(lines: Vec<String>) {
    let mut count_map: HashMap<i32, i32> = HashMap::new();
    let mut left_list: Vec<i32> = Vec::new();

    for line in lines {
        let line: Vec<&str> = line.split("   ").collect();
        for (index, num) in line.iter().enumerate() {
            let num: i32 = num.parse().unwrap();
            match index {
                0 => left_list.push(num),
                1 => *count_map.entry(num).or_insert(0) += 1,
                _ => println!("ANOTHER THING WRONG MY DUDE")
            }
        }
    }

    let mut total_similarity = 0;
    for num in left_list {
        let count = count_map.get(&num).copied().unwrap_or(0);
        let similarity = num * count;
        println!("num: {}, count: {}, similarty: {}, total_similarty: {}", num, count, similarity, total_similarity);
        total_similarity += similarity
    }

    println!("Totat Similarity: {}", total_similarity);

    
}

fn calc_distance_between(lines: Vec<String>) {
    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();
    
    for line in lines {
        let nums: Vec<&str> = line.split("   ").collect();
        for (index, num) in nums.iter().enumerate() {
            let num: i32 = num.parse().unwrap();
            match index {
                0 => left_heap.push(Reverse(num)),
                1 => right_heap.push(Reverse(num)),
                _ => println!("SOMETHING WRONG MY GUY")
            }
        }
    }
    let mut total_distance = 0;
    while !left_heap.is_empty() && !right_heap.is_empty() {
        let left_num = left_heap.pop();
        let right_num = right_heap.pop();
        let left_num = left_num.map(|Reverse(num)| num).unwrap_or(0);
        let right_num = right_num.map(|Reverse(num)| num).unwrap_or(0);
        let distance = (left_num - right_num).abs(); 
        println!("left: {}, right {}, distance {}, total {}", left_num, right_num, distance, total_distance);
        total_distance += distance;
    }
    
    println!("Total Distance: {}", total_distance);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}