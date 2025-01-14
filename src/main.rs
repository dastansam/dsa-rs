//! Word chain game

use std::{
    collections::{HashMap, HashSet, VecDeque},
    vec,
};

mod binary_tree;

/// A program which solves the "word chain game". A word chain is a collection of words created by mutating one letter from the previous one, e.g. cat -> sat -> sit.
/// In the word chain game you are given two words and asked to find the length of the shortest word chain which connects them (if one exists).
/// As input we are given a list of words along with the two to find the chain between. We assume all words in the given list are the same length, and are only made up of ASCII characters.
pub fn word_chain_game(start: &str, end: &str, words: &[String]) -> Option<usize> {
    let mut words_graph = HashMap::new();

    // for each word, mask one letter at each position to create a pattern that maps each words that are one letter away from each other
    for word in words {
        for i in 0..word.len() {
            let masked_pattern = format!("{}*{}", &word[..i], &word[i + 1..]);

            words_graph
                .entry(masked_pattern)
                .and_modify(|words: &mut Vec<&String>| words.push(word))
                .or_insert(vec![word]);
        }
    }

    // classic BFS
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0 as usize));
    while !queue.is_empty() {
        let (current_word, distance) = queue.pop_front().expect("always sth in the queue");

        if current_word == end {
            return Some(distance);
        }

        for i in 0..current_word.len() {
            let pattern = format!("{}*{}", &current_word[..i], &current_word[i + 1..]);

            if let Some(words) = words_graph.get(&pattern) {
                for word in words {
                    if !visited.contains(word) {
                        visited.insert(word);
                        queue.push_back((word, distance + 1));
                    }
                }
            } else {
                return None;
            }
        }
    }

    None
}

/// Read the word list from the given path
fn read_word_list(words_path: &str) -> Result<Vec<String>, std::io::Error> {
    let words = std::fs::read_to_string(words_path)?;
    let word_list = words.split('\n').map(Into::into).collect();
    Ok(word_list)
}

/// Accepts three arguments: the start word, the end word, and the path to the word list file
fn main() {
    let start = std::env::args().nth(1).expect("missing start word");
    let end = std::env::args().nth(2).expect("missing end word");
    let word_list_path = std::env::args().nth(3).expect("missing word list path");

    let word_list = read_word_list(&word_list_path).expect("failed to read word list");

    match word_chain_game(&start, &end, &word_list) {
        Some(distance) => println!(
            "The shortest word chain between {} and {} is {}",
            start, end, distance
        ),
        None => println!("No word chain exists between {} and {}", start, end),
    }
}

#[cfg(test)]
mod tests {
    use crate::{read_word_list, word_chain_game};
    const WORDS_PATH: &str = "res/three_letter_words.txt";

    #[test]
    fn test_path_exists() {
        let word_list = read_word_list(WORDS_PATH).unwrap();
        let start = "dog";
        let end = "imp";
        assert_eq!(word_chain_game(start, end, &word_list), Some(7));
    }

    #[test]
    fn test_path_does_not_exists() {
        let word_list = read_word_list(WORDS_PATH).unwrap();
        let start = "mad";
        let end = "gnu";
        assert_eq!(word_chain_game(start, end, &word_list), None);
    }
}
