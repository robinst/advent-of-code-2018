use std::collections::HashMap;
use std::io::{self, BufRead};

fn parse_u32_from(s: &str) -> u32 {
    let end = s.find(|c: char| !c.is_numeric()).unwrap_or(s.len());
    s[..end].parse().expect("Error parsing number")
}

fn parse_minute(s: &str) -> u32 {
    let colon = s.find(':').expect("Error parsing asleep line");
    parse_u32_from(&s[colon + 1..])
}

fn run<R: BufRead>(reader: &mut R) -> (u32, u32) {
    let mut lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Error reading line"))
        .filter(|l| !l.is_empty())
        .collect();
    lines.sort();

    let mut guard: u32 = 0;
    let mut start_minute: u32 = 0;

    let mut minutes = HashMap::new();
    let mut sums = HashMap::new();

    for line in lines {
        if let Some(hash) = line.find('#') {
            guard = parse_u32_from(&line[hash + 1..]);
        } else if line.contains("falls asleep") {
            start_minute = parse_minute(&line);
        } else if line.contains("wakes up") {
            let end_minute = parse_minute(&line);

            let mut guard_minutes = minutes.entry(guard).or_insert_with(|| HashMap::new());
            for minute in start_minute..end_minute {
                *guard_minutes.entry(minute).or_insert(0) += 1;
            }

            let sum = end_minute - start_minute;
            *sums.entry(guard).or_insert(0) += sum;
        }
    }

    let (best_guard, _) = sums
        .iter()
        .max_by_key(|(&_guard, &sum)| sum)
        .expect("No sums");
    let (best_minute, _) = minutes[best_guard]
        .iter()
        .max_by_key(|(&_minute, &freq)| freq)
        .expect("No minutes");
    (*best_guard, *best_minute)
}

fn main() {
    println!("Paste records");

    let (id, minute) = run(&mut io::stdin().lock());
    println!(
        "Guard {} at minute {}, multiplied: {}",
        id,
        minute,
        id * minute
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse() {
        let input = r#"
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
"#;

        let mut reader = Cursor::new(input);
        let (id, minutes) = run(&mut reader);
        assert_eq!(id, 10);
        assert_eq!(minutes, 24);
    }
}
