// Commented out code is a for a partially made potential optimization, but it's already decently fast
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let words: Vec<String> = {
        let file = File::open("./english.txt").unwrap();
        let reader = BufReader::new(file);
        reader.lines().map(|r| r.unwrap()).collect()
    };
    let board: Vec<char> = "rhreypcswnsntego".chars().collect();
    println!("{}", find_words(&words, &board, 4, 4).len());
}

fn find_words<'a, S: AsRef<str>>(
    dictionary: &'a [S],
    board: &[char],
    width: usize,
    height: usize,
) -> Vec<&'a str> {
    assert_eq!(width * height, board.len());
    let mut out = Vec::new();
    'word_loop: for word in dictionary.iter().map(|s| s.as_ref()) {
        if word.len() < 3 {
            continue;
        }
        /*let mut last_failure = String::from(" ");
        if word.starts_with(&last_failure) {
            continue;
        }*/
        for board_idx in 0..board.len() {
            match does_board_have_word(word, &mut Vec::new(), board_idx, board, width, height) {
                DoesBoardHaveWordResult::BoardHasWord => {
                    println!("{word}");
                    out.push(word);
                    continue 'word_loop;
                }
                DoesBoardHaveWordResult::BoardDoesNotHaveWord /*{ failed_at }*/ => {
                    /*println!("Failed at {failed_at} for {word}");*/
                }
            }
        }
    }
    out
}

fn neighbor_indexes(idx: usize, width: usize, height: usize) -> Vec<usize> {
    let x = idx % width;
    let y = idx / width;
    let can_go_left = x != 0;
    let can_go_right = x != (width - 1);
    let can_go_up = y != 0;
    let can_go_down = y != (height - 1);
    let mut out = Vec::new();
    if can_go_left {
        out.push(idx - 1);
    }
    if can_go_right {
        out.push(idx + 1);
    }
    if can_go_up {
        out.push(idx - width);
    }
    if can_go_down {
        out.push(idx + width);
    }
    if can_go_left && can_go_up {
        out.push(idx - width - 1);
    }
    if can_go_left && can_go_down {
        out.push(idx + width - 1);
    }
    if can_go_right && can_go_up {
        out.push(idx - width + 1);
    }
    if can_go_right && can_go_down {
        out.push(idx + width + 1);
    }
    out
}

#[derive(PartialEq, Eq)]
enum DoesBoardHaveWordResult {
    BoardHasWord,
    BoardDoesNotHaveWord, /* { failed_at: usize }*/
}

fn does_board_have_word(
    word: &str,
    visited: &mut Vec<usize>,
    next_index: usize,
    board: &[char],
    width: usize,
    height: usize,
) -> DoesBoardHaveWordResult {
    // let mut longest_visited_len = 0;
    for neighbor_index in neighbor_indexes(next_index, width, height) {
        if visited.contains(&neighbor_index) {
            continue;
        }
        if board[neighbor_index] == word.chars().next().unwrap() {
            if word.len() == 1 {
                return DoesBoardHaveWordResult::BoardHasWord;
            } else {
                visited.push(next_index);
                match does_board_have_word(
                    &word[1..],
                    visited,
                    neighbor_index,
                    board,
                    width,
                    height,
                ) {
                    DoesBoardHaveWordResult::BoardHasWord => {
                        return DoesBoardHaveWordResult::BoardHasWord;
                    }
                    DoesBoardHaveWordResult::BoardDoesNotHaveWord /*{ failed_at }*/ => {
                        /*if failed_at > longest_visited_len {
                            longest_visited_len = failed_at;
                        }
                        if visited.len() > longest_visited_len {
                            longest_visited_len = visited.len();
                        }*/
                        visited.pop();
                    }
                }
            }
        }
    }
    DoesBoardHaveWordResult::BoardDoesNotHaveWord /*{
                                                      failed_at: longest_visited_len,
                                                  }*/
}
