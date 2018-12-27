use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Step(char);

impl Step {
    fn new(s: &str) -> Step {
        assert_eq!(s.len(), 1);
        Step(
            s.chars()
                .next()
                .expect("Expected a single character for a step key"),
        )
    }

    fn seconds(&self) -> u32 {
        // A is 1, B is 2, etc
        (self.0 as u32) - ('A' as u32) + 1
    }
}

fn total_time(requirements: &[(Step, Step)], num_workers: usize, base_seconds: u32) -> u32 {
    let mut candidates: BTreeMap<&Step, u32> =
        requirements.iter().map(|(a, _b)| (a, 0u32)).collect();

    let mut incoming = HashMap::new();
    let mut outgoing = HashMap::new();
    for (a, b) in requirements {
        outgoing.entry(a).or_insert_with(|| Vec::new()).push(b);
        incoming.entry(b).or_insert_with(|| Vec::new()).push(a);

        // Needs another step to finish first, remove from initial candidates
        candidates.remove(&b);
    }

    let mut workers = vec![0u32; num_workers];

    while !candidates.is_empty() {
        let candidate = {
            candidates
                .iter()
                .filter(|&(step, _)| {
                    if let Some(prerequisites) = incoming.get(step) {
                        // If all prerequisites have been done, we can use the candidate
                        prerequisites.iter().all(|p| !outgoing.contains_key(p))
                    } else {
                        // No prerequisite steps, we can use it
                        true
                    }
                })
                .min_by_key(|(_, start)| *start)
                .map(|(step, start)| (step.clone(), start.clone()))
        };

        if let Some((step, start)) = candidate {
            let worker = workers.iter_mut().min().expect("No workers available");
            let worker_start = start.max(*worker);
            let seconds = step.seconds() + base_seconds;
            // println!("Step {} starting at {} for {} s", step.0, worker_start, seconds);
            let next_start = worker_start + seconds;
            *worker = next_start;

            if let Some(nodes) = outgoing.get(&step) {
                for &node in nodes {
                    let candidate_start = candidates.entry(node.clone()).or_default();
                    // Earliest we can start this dependent step is when all prerequisites have
                    // finished in a worker.
                    *candidate_start = next_start.max(*candidate_start);
                }
            }
            outgoing.remove(&step);
            candidates.remove(&step);
        }
    }

    workers.iter().max().map(|max| *max).unwrap_or(0)
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

    let result = total_time(&requirements, 5, 60);
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
        assert_eq!(total_time(&requirements, 2, 0), 15);
    }
}
