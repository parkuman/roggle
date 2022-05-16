mod board;
mod trie;

use board::Board;
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

pub fn solve(board_string: &str) -> &str {
  let word_list = lines_from_file("word_lists/words_alpha.txt");
  let mut trie = Trie::from(word_list);

  let board = Board::from(board_string);
  board.print();

  println!("solutionset: {:?}", board.solve(&mut trie));

  board_string
}
