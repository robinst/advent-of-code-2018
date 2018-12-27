use std::collections::BTreeSet;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Step(String);

impl Step {
    fn new(s: &str) -> Step {
        Step(s.to_string())
    }
}

fn order(requirements: &[(Step, Step)]) -> String {
    let mut candidates: BTreeSet<_> = requirements.iter().map(|(a, _b)| a).collect();

    let mut incoming = HashMap::new();
    let mut outgoing = HashMap::new();
    for (a, b) in requirements {
        outgoing.entry(a).or_insert_with(|| Vec::new()).push(b);
        incoming.entry(b).or_insert_with(|| Vec::new()).push(a);

        // Needs another step to finish first, remove from initial candidates
        candidates.remove(b);
    }

    let mut result = String::new();

    while !candidates.is_empty() {
        let candidate = {
            candidates
                .iter()
                .find(|&c| {
                    if let Some(prerequisites) = incoming.get(c) {
                        // If all prerequisites have been done, we can use the candidate
                        prerequisites.iter().all(|p| !outgoing.contains_key(p))
                    } else {
                        // No prerequisite steps, we can use it
                        true
                    }
                })
                .map(|k| k.clone())
        };
        if let Some(key) = candidate {
            if let Some(nodes) = outgoing.get(&key) {
                for &node in nodes {
                    candidates.insert(node);
                }
            }
            outgoing.remove(&key);
            candidates.remove(&key);
            result.push_str(&key.0);
        }
    }

    result
}

fn main() {
    println!("Paste input");

    let mut requirements = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line.expect("Error reading line");
        // format: "Step C must be finished before step A can begin."
        let parts: Vec<_> = line.split(" ").collect();
        if parts.len() == 10 {
            let from = parts[1];
            let to = parts[7];
            requirements.push((Step::new(from), Step::new(to)));
        }
    }

    let result = order(&requirements);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let requirements: Vec<_> = vec![
            ("C", "A"),
            ("C", "F"),
            ("A", "B"),
            ("A", "D"),
            ("B", "E"),
            ("D", "E"),
            ("F", "E"),
        ]
        .iter()
        .map(|(a, b)| (Step::new(a), Step::new(b)))
        .collect();
        assert_eq!(order(&requirements), "CABDFE".to_string());
    }
}
