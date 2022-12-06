use std::collections::HashSet;

use anyhow::Result;

#[derive(Debug)]
struct Window {
    start: usize,
    end: usize
}

fn is_marker(s: &str) -> bool {
    let mut hs: HashSet<char> = HashSet::new();
    for c in s.chars() {
        hs.insert(c);
    }
    return hs.len() == s.len();
}

fn main() -> Result<()> {
    let input = include_str!("input6.prod");

    let mut start_of_packet_windows: Vec<Window> = Vec::new();
    let mut start_of_message_windows: Vec<Window> = Vec::new();


    for i in 0..input.len() {
        let w1 = Window {
            start: i,
            end: i+3,
        };
        if w1.end >= input.len() {
            continue;
        }
        start_of_packet_windows.push(w1);
        let w2 = Window {
            start: i,
            end: i+13,
        };
        if w2.end >= input.len() {
            continue;
        }
        start_of_message_windows.push(w2);
    }

    for window in start_of_packet_windows {
        let chunk = &input[window.start ..= window.end];
        if is_marker(chunk) {
            // println!("first window: {:?}", window);
            // println!("first chunk: {}", chunk);
            println!("start of packet marker: {}", window.end + 1);
            break;
        }
    }

    for window in start_of_message_windows {
        let chunk = &input[window.start ..= window.end];
        if is_marker(chunk) {
            // println!("first window: {:?}", window);
            // println!("first chunk: {}", chunk);
            println!("start of message marker: {}", window.end + 1);
            break;
        }
    }
    return Ok(());
}
