use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let mut seen = HashSet::new();

    println!("Paste boxes");
    for line in io::stdin().lock().lines() {
        let line = line.expect("Error reading line");

        let char_count = line.chars().count();
        for wildcard in 0..char_count {
            let pattern: String = line
                .chars()
                .enumerate()
                .map(|(i, c)| if i == wildcard { ' ' } else { c })
                .collect();
            if seen.contains(&pattern) {
                let without_wildcard = pattern.replace(' ', "");
                println!("Found box: {}", without_wildcard);
                return;
            }

            seen.insert(pattern);
        }
    }
}
