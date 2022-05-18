# roggle

A Boggle solver written in Rust! Simply pass in an N x M board as a string with rows separated by spaces. For the Qu tile simply put `q` as the letter.

# Example Usage

```rust
use roggle;
use std::collections::HashSet;

fn main() {
  let board = "wodp djik asop saps";
  let solutions: HashSet<String> = roggle::solve(board);
  println!("{:?}", solutions);
}
```
