use roggle;

fn main() {
  let board = "1 2 3 4";
  println!("Board: {}", board);
  println!("Solved: {}", roggle::solve(&board));
}
