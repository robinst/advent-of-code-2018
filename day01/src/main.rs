use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    println!("Paste instructions");
    let mut instructions = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Error reading line");
        let number: i32 = line.parse().expect("Error parsing number");
        instructions.push(number);
    }

    println!("Got {} instructions", instructions.len());

    let mut seen = HashSet::new();
    let mut calculations = 0;

    let mut state = 0;
    loop {
        for instruction in &instructions {
            state += instruction;
            calculations += 1;

            if seen.contains(&state) {
                println!("Found number {} after {} calculations", state, calculations);
                return;
            }

            seen.insert(state);
        }
    }
}
