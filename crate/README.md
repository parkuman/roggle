<div align="center">
  <a href="https://crates.io/crates/roggle" target="_blank" rel="noopener noreferrer"><img width="400" src="https://prowe.ca/images/projects/roggle/roggle.png" alt="roggle logo"></a>

  <h1>🦀 roggle 🔡</h1>

  <p>
    <strong>A Boggle Solver Crate!</strong>
  </p>

  <p>
    <a target="_blank" rel="noopener noreferrer" href="https://crates.io/crates/roggle"><img alt="crates.io" src="https://img.shields.io/crates/v/roggle"/></a>
    <a target="_blank" rel="noopener noreferrer" href="https://docs.rs/roggle/"><img alt="Docs" src="https://img.shields.io/badge/docs.rs-roggle-green"/></a>
  </p>

  <h3>
    <a target="_blank" rel="noopener noreferrer" href="https://www.npmjs.com/package/roggle">NPM Package</a>
    <span> | </span>
    <a target="_blank" rel="noopener noreferrer" href="https://roggle.prowe.ca/">Web Demo</a>
  </h3>

</div>

## About

Have you ever been playing Boggle and wished you could know all the solutions on the board? Wish no longer! Roggle is a Rust crate that allows you to solve any N x M Boggle board.

Simply pass in an N x M board as a string with rows separated by spaces. For the `Qu` tile put `q` as the letter.



## Example Usage
### Board
```
 w  o  d  p
 d  j  i  k
 a  s  o  p
 s  a  Qu s
```

### Code
```rust
use roggle;
use std::collections::HashSet;

fn main() {
  let board = "wodp djik asop saqs";
  let solutions: HashSet<String> = roggle::solve(board);
  println!("{:?}", solutions);
}
```

## Under the Hood
Roggle uses a Trie to breakdown the english dictionary into an easily searchable tree. It then recurses over each board tile, searching all neighbours for possible words until it finishes! 

## Dictionary
The dictionary used is a [466k word english dictionary](https://github.com/dwyl/english-words). Some words are not super common (`aaaa` is a word apparently??), but Roggle would rather show you all possibilities then leave you in the dust with less points! 

As with any game of boggle, feel free to argue which words are legal with your friends :)
