use intrusive_collections::{intrusive_adapter, LinkedList, LinkedListLink};
use std::io;

#[derive(Debug)]
struct Unit {
    link: LinkedListLink,
    value: char,
}

impl Unit {
    fn new(value: char) -> Box<Unit> {
        Box::new(Unit {
            link: LinkedListLink::new(),
            value,
        })
    }

    fn react(&self, other: &Unit) -> bool {
        self.value != other.value && self.value.eq_ignore_ascii_case(&other.value)
    }
}

intrusive_adapter!(UnitAdapter = Box<Unit>: Unit { link: LinkedListLink });

fn run(line: &str) -> String {
    let mut list = LinkedList::new(UnitAdapter::new());
    for unit in line.chars().map(Unit::new) {
        list.push_back(unit);
    }

    let mut cursor = list.cursor_mut();
    cursor.move_next();

    while !cursor.is_null() {
        if let Some(a) = cursor.get() {
            if let Some(b) = cursor.peek_next().get() {
                if a.react(b) {
                    cursor.remove();
                    cursor.remove();
                    if !cursor.peek_prev().is_null() {
                        // Due to removing the current units, maybe the earlier unit will now react
                        // with the next one, so check it again.
                        cursor.move_prev();
                    }
                    continue;
                }
            }
        }
        cursor.move_next();
    }

    list.iter().map(|u| u.value).collect()
}

fn main() {
    println!("Paste input");

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Error reading line");

    let result = run(&line);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("aA"), "");
        assert_eq!(run("abBA"), "");
        assert_eq!(run("abAB"), "abAB");
        assert_eq!(run("aabAAB"), "aabAAB");
        assert_eq!(run("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
        assert_eq!(run("abA"), "abA");
        assert_eq!(run("bBkK"), "");
        assert_eq!(run("bBkKQqgaAGzyYZCc"), "");
    }
}
