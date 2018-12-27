use std::io;
use std::io::BufRead;
use std::collections::LinkedList;

enum Entry {
    Node,
    Metadata
}

// format:
//
// 2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
// A----------------------------------
//     B----------- C-----------
//                      D-----
fn sum_metadata(input: &[u32]) -> u32 {
    let mut iter = input.iter();

    let mut state = LinkedList::new();
    state.push_back(Entry::Node);

    let mut sum = 0;
    while let Some(entry) = state.pop_front() {
        match entry {
            Entry::Node => {
                let nodes = *iter.next().expect("Expected number of child nodes");
                let metadata_entries = *iter.next().expect("Expected number of metadata entries");
                // Push metadata and then nodes, so that nodes are processed first
                for _ in 0..metadata_entries {
                    state.push_front(Entry::Metadata);
                }
                for _ in 0..nodes {
                    state.push_front(Entry::Node);
                }
            },
            Entry::Metadata => {
                let value = iter.next().expect("Expected metadata entry");
                sum += value;
            }
        }
    }
    sum
}

fn main() {
    println!("Paste input");

    let mut input = Vec::new();

    for line in io::stdin().lock().lines() {
        let line = line.expect("Error reading input");
        for numeric in line
            .split(|c: char| !c.is_numeric())
            .filter(|s| !s.is_empty()) {
            let number = numeric.parse().expect("Error parsing number");
            input.push(number);
        }
    }

    let result = sum_metadata(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [2u32, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        assert_eq!(sum_metadata(&input), 138);
    }
}
