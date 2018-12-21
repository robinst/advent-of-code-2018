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

// format: #1 @ 817,273: 26x26
//              ^   ^    ^  ^
//              x   y    w  h
fn parse(line: &str) -> Option<Claim> {
    let at = line.find(" @ ")?;
    let after_at = &line[at + 3..];

    let id = line[1..at].parse().ok()?;

    let comma = after_at.find(',')?;
    let colon = after_at.find(": ")?;

    let x: u32 = after_at[..comma].parse().ok()?;
    let y: u32 = after_at[comma + 1..colon].parse().ok()?;

    let after_colon = &after_at[colon + 2..];
    let cross = after_colon.find('x')?;
    let w: u32 = after_colon[..cross].parse().ok()?;
    let h: u32 = after_colon[cross + 1..].parse().ok()?;

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
