use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

// format: #1 @ 817,273: 26x27
//          ^   ^   ^    ^  ^
//          id  x   y    w  h
fn parse(line: &str) -> Option<Claim> {
    let mut numbers = line
        .split(|c: char| !c.is_numeric())
        .filter(|s| !s.is_empty());

    let id = numbers.next()?.parse().ok()?;
    let x = numbers.next()?.parse().ok()?;
    let y = numbers.next()?.parse().ok()?;
    let w = numbers.next()?.parse().ok()?;
    let h = numbers.next()?.parse().ok()?;
    Some(Claim { id, x, y, w, h })
}

fn main() {
    let mut seen = HashMap::new();
    let mut candidates = HashSet::new();

    println!("Paste claims");
    for line in io::stdin().lock().lines() {
        let line = line.expect("Error reading line");

        if let Some(Claim { id, x, y, w, h }) = parse(&line) {
            // println!("{},{} {}x{}", x, y, w, h);
            candidates.insert(id);

            for cx in x..x + w {
                for cy in y..y + h {
                    let point = (cx, cy);
                    if let Some(seen_ids) = seen.get(&point) {
                        for seen_id in seen_ids {
                            candidates.remove(seen_id);
                        }
                        candidates.remove(&id);
                    }
                    seen.entry(point).or_insert_with(|| Vec::new()).push(id);
                }
            }
        }
    }

    println!("Remaining candidates: {:?}", candidates);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let result = parse("#1 @ 817,273: 26x27");
        let Claim { id, x, y, w, h } = result.unwrap();

        assert_eq!(id, 1);
        assert_eq!(x, 817);
        assert_eq!(y, 273);
        assert_eq!(w, 26);
        assert_eq!(h, 27);
    }
}
