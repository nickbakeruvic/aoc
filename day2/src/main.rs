use std::fs;

fn main() {
    let data = fs::read_to_string("src/input.in").expect("Unable to read file");
    let nums = parse_to_array(data);

    let mut safe = 0;
    for line in nums.iter() {
        for i in 0..line.len() {
            let mut removed = line.clone();
            removed.remove(i);
            if check_safe(removed) {
                safe += 1;
                break
            }
        }
    }

    println!("Safe count: {}", safe);
}

fn check_safe(line: Vec<i32>) -> bool {
    let increasing = line.windows(2).all(|n| n[0] < n[1]);
    let decreasing = line.windows(2).all(|n| n[0] > n[1]);
    let difference = line.windows(2).all(|n| (n[0] - n[1]).abs() >= 1 && (n[0] - n[1]).abs() <= 3);

    (increasing || decreasing) && difference
}

fn parse_to_array(data: String) -> Vec<Vec<i32>> {
    data
        .lines()
        .map(|line|
            line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect()
        )
        .collect()
}