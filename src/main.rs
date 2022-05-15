use roggle;
use roggle::Trie;

fn main() {
  // let board = "1 2 3 4";
  // println!("Board: {}", board);
  // println!("Solved: {}", roggle::solve(&board));
  let mut trie = Trie::new();
  trie.insert("hello");
  trie.insert("donald");
  println!("{}", trie.find("hello"));
  println!("{}", trie.find("hell"));
  println!("{}", trie.find("h"));
  println!("{}", trie.find("donald"));
  println!("{}", trie.find("donal"));
  println!("{}", trie.find("parker"));
  println!("{}", trie.find("hellothere"));
}
