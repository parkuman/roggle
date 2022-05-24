import init, { solve } from "./pkg/roggle.js";

function solve_board() {
  const board = "efkj keps aoeq";
  const start = new Date();
  console.log("Solving...");
  console.log(solve(board));
  const end = new Date();
  console.log("completed in", (end - start) / 1000, "seconds");
}

init().then(solve_board);
