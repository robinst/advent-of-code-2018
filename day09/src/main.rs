use std::collections::VecDeque;
use std::env;

fn main() {
    let players: usize = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .expect("Specify the number of players");
    let last_marble: u32 = env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .expect("Specify the last marble");

    let mut circle = VecDeque::new();
    circle.push_back(0);

    let mut current = 0;
    let mut marble = 1;
    let mut player = 0; // 0 based players
    let mut scores: Vec<u32> = Vec::with_capacity(players);
    scores.resize(players, 0);

    while marble <= last_marble {
        if marble % 23 == 0 {
            scores[player] += marble;
            let mut remove = current;
            for _ in 0..7 {
                if remove == 0 {
                    remove = circle.len() - 1;
                } else {
                    remove -= 1;
                }
            }
            if let Some(removed) = circle.remove(remove) {
                scores[player] += removed;
            }
            current = remove % circle.len();
        } else {
            let new = ((current + 1) % circle.len()) + 1;
            circle.insert(new, marble);

            current = new;
        }

        // println!("[{}] {:?}", player + 1, circle);
        marble += 1;
        player = (player + 1) % players;
    }

    if let Some(max) = scores.iter().max() {
        println!("High score: {}", max);
    }
}
