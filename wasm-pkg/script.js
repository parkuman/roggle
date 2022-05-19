import init, { solve } from "./pkg/roggle.js";

function solve_board() {
  const board = "qeen zzzz";

  console.log("Solving...");
  console.log(solve(board));
}

init().then(solve_board);
