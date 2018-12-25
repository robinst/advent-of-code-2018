use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::u32;

fn manhattan_distance((x1, y1): &(u32, u32), (x2, y2): &(u32, u32)) -> u32 {
    let dist_x = if x1 >= x2 { x1 - x2 } else { x2 - x1 };
    let dist_y = if y1 >= y2 { y1 - y2 } else { y2 - y1 };
    dist_x + dist_y
}

fn largest_area(coordinates: &[(u32, u32)]) -> u32 {
    let min_x = coordinates.iter().min_by_key(|(x, _)| x).unwrap().0;
    let min_y = coordinates.iter().min_by_key(|(_, y)| y).unwrap().1;
    let max_x = coordinates.iter().max_by_key(|(x, _)| x).unwrap().0;
    let max_y = coordinates.iter().max_by_key(|(_, y)| y).unwrap().1;

    let mut infinites = HashSet::new();
    let mut area: HashMap<usize, u32> = HashMap::new();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let mut closest_coord = None;
            let mut closest_dist = u32::MAX;
            for (i, coord) in coordinates.iter().enumerate() {
                let dist = manhattan_distance(coord, &(x, y));
                if dist < closest_dist {
                    closest_coord = Some(i);
                    closest_dist = dist;
                } else if dist == closest_dist {
                    closest_coord = None;
                }
            }

            if let Some(closest) = closest_coord {
                let infinite = x == min_x || x == max_x || y == min_y || y == max_y;
                if infinite {
                    infinites.insert(closest);
                } else {
                    *area.entry(closest).or_insert(0) += 1;
                }
            }
        }
    }

    for infinite in infinites {
        area.remove(&infinite);
    }

    if let Some(&max_count) = area.values().max_by_key(|&count| count) {
        max_count
    } else {
        0
    }
}

fn main() {
    println!("Paste input");

    let mut coordinates = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line.expect("Error reading line");
        // format: 1, 6
        let parts: Vec<_> = line.split(", ").collect();
        if parts.len() == 2 {
            let x: u32 = parts[0].parse().expect("Error parsing coordinate");
            let y: u32 = parts[1].parse().expect("Error parsing coordinate");
            coordinates.push((x, y));
        }
    }

    let result = largest_area(&coordinates);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(
            largest_area(&[(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)]),
            17
        );
    }
}
