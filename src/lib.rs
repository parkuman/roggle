mod board;
mod trie;

use board::Board;
use std::process::exit;
use trie::Trie;

use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
  let f = File::open(filename);
  match f {
    Ok(file) => {
      let reader = BufReader::new(&file);
      reader
        .lines()
        .map(|result| result.unwrap())
        .filter(|ref line| line.len() >= 3)
        .collect()
      // .filter(|ref line| {
      //   let mut iter = line.chars();
      //   while let Some(c) = iter.next() {
      //     if c == 'q' || c == 'q' {
      //       if let Some(n) = iter.next() {
      //         if !(n == 'u' || n == 'U') {
      //           // q not followed by u
      //           return false;
      //         }
      //       } else {
      //         // line ends in q
      //         return false;
      //       }
      //     }
      //   }
      //   true
      // })
      // .map(|ref line| line.replace("qu", "q"))
    }
    Err(e) => panic!("{:?}", e),
  }
}

fn validate_board(board_input: &str) -> &str {
  if board_input.is_empty() {
    println!("Empty board provided!");
    exit(1);
  }

  let board_rows: Vec<&str> = board_input.split(" ").collect();
  let mut max_row_len: Option<usize> = None; // a running tracker to make sure all rows are the same length

  for row in board_rows {
    let row_len = row.chars().collect::<Vec<char>>().len();

    match max_row_len {
      None => {
        // if max_row_len doesn't have a value, set it to the current row length.
        max_row_len = Some(row_len);
      }
      Some(i) => {
        // if max_row_len has a value, ensure that this new row we're checking is the same size as it. if it is, set max_row_len and proceed
        if i != row_len {
          println!(
            "rows must be of equal size! row \"{}\" is not the same length as the rest",
            row
          );
          exit(1);
        } else {
          max_row_len = Some(i);
        }
      }
    }
  }

  board_input
}

pub fn solve(input_board: &str) -> Vec<String> {
  let board_string = input_board.trim();
  validate_board(board_string);

  let word_list = include_str!("../word_lists/words_alpha.txt")
    .split("\r\n")
    .map(|str| str.to_string())
    .filter(|ref line| line.len() >= 3) // filter out words less than 3 letters
    // TODO: replace qu with q and handle it later
    .collect();

  let mut trie = Trie::from(word_list);

  let board = Board::from(board_string);

  let solution_set = board.solve(&mut trie);

  solution_set
}
