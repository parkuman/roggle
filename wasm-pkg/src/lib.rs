mod utils;

use roggle;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn solve(boggle_board: String) -> String {
  let solution_set = roggle::solve(&boggle_board)
    .into_iter()
    .collect::<Vec<String>>()
    .join(", ");

  return solution_set;
}
