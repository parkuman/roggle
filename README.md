<div align="center">
  <img width="400" src="https://prowe.ca/images/projects/roggle/roggle.png" alt="roggle logo">

  <h1>🦀 roggle 🔡</h1>

  <p>
    <strong>A Boggle Solver written in Rust and Compiled to WebAssembly!</strong>
  </p>

  <p>
    <a href="https://www.npmjs.com/package/roggle"><img alt="crates.io" src="https://badge.fury.io/js/roggle.svg"/></a>
    <a target="_blank" rel="noopener noreferrer" href="https://crates.io/crates/roggle"><img alt="crates.io" src="https://img.shields.io/crates/v/roggle"/></a>
    <a target="_blank" rel="noopener noreferrer" href="https://docs.rs/roggle/"><img alt="Docs" src="https://img.shields.io/badge/docs.rs-roggle-green"/></a>
   
  </p>

  <h3>
    <a target="_blank" rel="noopener noreferrer" href="https://github.com/parkuman/roggle/tree/main/crate">Crate Source</a>
    <span> | </span>
    <a target="_blank" rel="noopener noreferrer" href="https://github.com/parkuman/roggle/tree/main/wasm-pkg">Package Source</a>
    <span> | </span>
    <a target="_blank" rel="noopener noreferrer" href="https://roggle.prowe.ca/">Demo</a>
  </h3>

</div>

## About

Have you ever been playing Boggle and wished you could know all the solutions on the board? Wish no longer! Roggle is a Rust crate + NPM package that allows you to solve any N x M Boggle board.

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
#### Rust
```rust
use roggle;
use std::collections::HashSet;

fn main() {
  let board = "wodp djik asop saqs";
  let solutions: HashSet<String> = roggle::solve(board);
  println!("{:?}", solutions);
}
```
#### JavaScript
```javascript
import init, { solve } from "roggle";

function solve_board() {
  const board = "wodp djik asop saqs";
  console.log(solve(board));
}

init().then(solve_board);
```

## Under the Hood
Roggle is a Rust-based Boggle solver compiled to WebAssembly, allowing us to run it in the browser! For more info on the actual Rust implementation, check out the [roggle crate on crates.io.](https://crates.io/crates/roggle) 

Roggle uses a Trie to breakdown the english dictionary into an easily searchable tree. It then recurses over each board tile, searching all neighbours for possible words until it finishes! 

## Dictionary
The dictionary used is a [466k word english dictionary](https://github.com/dwyl/english-words). Some words are not super common (`aaaa` is a word apparently??), but Roggle would rather show you all possibilities then leave you in the dust with less points! 

As with any game of boggle, feel free to argue which words are legal with your friends :)
