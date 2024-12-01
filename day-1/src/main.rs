use std::fs::read_to_string;
use std::collections::BinaryHeap;
use std::cmp::Reverse;


fn main() {
    let lines = read_lines("./input.txt");

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

    calc_distance_between(left_heap.clone(), right_heap.clone());
}

fn calc_similarity_score(mut left_heap: BinaryHeap<Reverse<i32>>, mut right_heap: BinaryHeap<Reverse<i32>>) {

}

fn calc_distance_between(mut left_heap: BinaryHeap<Reverse<i32>>, mut right_heap: BinaryHeap<Reverse<i32>>) {
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
    
    println!("{}", total_distance);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}