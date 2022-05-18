# roggle

A Boggle solver written in Rust! This is still very much a WIP while I actually learn how to use Rust.

# Example Usage

```rust
extern crate roggle;

fn main() {
  let board = "wodp djik asop saps";
  let solutions: Vec<String> = roggle::solve(board);
  println!("{:?}", solutions);
}
```
