use intrusive_collections::{intrusive_adapter, LinkedList, LinkedListLink};
use std::env;

#[derive(Debug)]
struct Marble {
    link: LinkedListLink,
    value: u32,
}

impl Marble {
    fn new(value: u32) -> Box<Marble> {
        Box::new(Marble {
            link: LinkedListLink::new(),
            value,
        })
    }
}

intrusive_adapter!(MarbleAdapter = Box<Marble>: Marble { link: LinkedListLink });

fn main() {
    let players: usize = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .expect("Specify the number of players");
    let last_marble: u32 = env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .expect("Specify the last marble");

    let mut circle = LinkedList::new(MarbleAdapter::new());
    circle.push_back(Marble::new(0));

    let mut cursor = circle.cursor_mut();
    cursor.move_next();
    let mut marble = 1;
    let mut player = 0; // 0 based players
    let mut scores: Vec<u32> = Vec::with_capacity(players);
    scores.resize(players, 0);

    while marble <= last_marble {
        if marble % 23 == 0 {
            scores[player] += marble;
            for _ in 0..7 {
                cursor.move_prev();
                if cursor.is_null() {
                    cursor.move_prev();
                }
            }
            if let Some(removed) = cursor.remove() {
                scores[player] += removed.value;
            }
        } else {
            cursor.move_next();
            if cursor.is_null() {
                cursor.move_next();
            }
            cursor.insert_after(Marble::new(marble));
            cursor.move_next();
        }

        marble += 1;
        player = (player + 1) % players;
    }

    if let Some(max) = scores.iter().max() {
        println!("High score: {}", max);
    }
}
