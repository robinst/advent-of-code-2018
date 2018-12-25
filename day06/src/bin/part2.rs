use std::io;
use std::io::BufRead;
use std::u32;

fn manhattan_distance((x1, y1): &(u32, u32), (x2, y2): &(u32, u32)) -> u32 {
    let dist_x = if x1 >= x2 { x1 - x2 } else { x2 - x1 };
    let dist_y = if y1 >= y2 { y1 - y2 } else { y2 - y1 };
    dist_x + dist_y
}

fn inner_region_size(coordinates: &[(u32, u32)], less_than_distance: u32) -> u32 {
    let min_x = coordinates.iter().min_by_key(|(x, _)| x).unwrap().0;
    let min_y = coordinates.iter().min_by_key(|(_, y)| y).unwrap().1;
    let max_x = coordinates.iter().max_by_key(|(x, _)| x).unwrap().0;
    let max_y = coordinates.iter().max_by_key(|(_, y)| y).unwrap().1;

    let mut region_size: u32 = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let dist_sum: u32 = coordinates
                .iter()
                .map(|coord| manhattan_distance(coord, &(x, y)))
                .sum();
            if dist_sum < less_than_distance {
                region_size += 1;
            }
        }
    }

    region_size
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

    let result = inner_region_size(&coordinates, 10000);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(
            inner_region_size(&[(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)], 32),
            16
        );
    }
}
