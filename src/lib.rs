mod board;
pub mod trie;

use board::Board;
// use trie::Trie;

pub fn solve(board_string: &str) -> &str {
  // let word_list: Vec<&str> = vec!["apple", "banana"];
  // let trie = Trie::from(word_list);
  // println!("{:?}", trie);

  let board = Board::from(board_string);
  board.print();

  println!("solutionset: {:?}", board.solve());

  board_string
}
