use roggle;
use std::env::args;
use std::process::exit;

fn main() {
  let args: Vec<String> = args().collect();

  match args.len() {
    1 => {
      println!("No board argument provided!");
      exit(1);
    }
    _ => (),
  }

  let board = &args[1];

  println!("solutions: {:?}", roggle::solve(board));
}
