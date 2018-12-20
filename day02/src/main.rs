use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut twos = 0u32;
    let mut threes = 0u32;

    println!("Paste boxes");
    for line in io::stdin().lock().lines() {
        let line = line.expect("Error reading line");

        let mut counts = HashMap::new();
        for c in line.chars() {
            let count = counts.entry(c).or_insert(0u32);
            *count += 1;
        }

        if counts.values().any(|v| *v == 2) {
            twos += 1;
        }
        if counts.values().any(|v| *v == 3) {
            threes += 1;
        }
    }

    println!("Checksum: {}", twos * threes);
}
