use std::fs::read_to_string;
use itertools::Itertools;


fn main() {
    let lines = read_lines("./input.txt");

    get_safe_count(lines.clone());
    get_safe_count_with_dampener(lines);
}

fn get_safe_count(lines: Vec<String>) {
    let mut total_count = 0;

    for line in lines {
        let nums: Vec<i16> = line.split(" ").map(|v| v.parse::<i16>().unwrap()).collect();
        let is_safe = is_report_safe(nums);

        if is_safe {
            total_count += 1
        }
    }

    println!("Total Count: {}", total_count)
}

fn get_safe_count_with_dampener(lines: Vec<String>) {
    let mut total_count = 0;
    
    for line in lines {
        let nums: Vec<i16> = line.split(" ").map(|v| v.parse::<i16>().unwrap()).collect();
        let mut is_safe = is_report_safe(nums.clone());
        if !is_safe {
            is_safe = nums.clone().into_iter().combinations(nums.len() - 1).any(|combination| is_report_safe(combination));
        }
        
        if is_safe {
            total_count += 1
        }
    }
    
    println!("Total Count: {}", total_count)
}

fn is_report_safe(nums: Vec<i16>) -> bool {
    let mut is_safe = true;
    let mut direction = "unknown";
    for (prev, curr) in nums.iter().zip(nums.iter().skip(1)) {
        if !is_safe {
            break;
        }

        if direction == "unknown" {
            if curr > prev {
                direction = "asc"
            } else if curr < prev {
                direction = "desc"
            } else {
                is_safe = false;
                continue;
            }
        }

        is_safe = direction_safe_condition(direction, *prev, *curr) && diff_safe_condition(*prev, *curr);
    }

    return is_safe
}


fn direction_safe_condition(direction: &str, left: i16, right: i16) -> bool {
    if direction == "asc" {
        if left < right {
            return true
        }
    } else if direction == "desc" {
        if left > right {
            return true
        }
    }
    
    return false
}

fn diff_safe_condition(left: i16, right:i16) -> bool {
    let diff = (right - left).abs();
    if diff > 3 || diff == 0 {
        return false
    }

    return true
}




fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}