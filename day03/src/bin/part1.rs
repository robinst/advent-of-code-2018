use std::collections::HashSet;
use std::io::{self, BufRead};

// format: #1 @ 817,273: 26x26
//              ^   ^    ^  ^
//              x   y    w  h
fn parse(line: &str) -> Option<((u32, u32), (u32, u32))> {
    let at = line.find("@ ")?;
    let after_at = &line[at + 2..];
    let comma = after_at.find(',')?;
    let colon = after_at.find(": ")?;

    let x: u32 = after_at[..comma].parse().ok()?;
    let y: u32 = after_at[comma + 1..colon].parse().ok()?;

    let after_colon = &after_at[colon + 2..];
    let cross = after_colon.find('x')?;
    let w: u32 = after_colon[..cross].parse().ok()?;
    let h: u32 = after_colon[cross + 1..].parse().ok()?;

    Some(((x, y), (w, h)))
}

fn main() {
    let mut seen = HashSet::new();
    let mut overlaps = HashSet::new();

    println!("Paste claims");
    for line in io::stdin().lock().lines() {
        let line = line.expect("Error reading line");

        if let Some(((x, y), (w, h))) = parse(&line) {
            // println!("{},{} {}x{}", x, y, w, h);

            for cx in x..x + w {
                for cy in y..y + h {
                    if seen.contains(&(cx, cy)) {
                        overlaps.insert((cx, cy));
                    }
                    seen.insert((cx, cy));
                }
            }
        }
    }

    println!("Overlaps: {}", overlaps.len());
}
