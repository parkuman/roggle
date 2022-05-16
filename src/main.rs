use roggle;
use std::env::args;
use std::process::exit;

fn validate_board(board_input: &str) -> &str {
  let board_rows: Vec<&str> = board_input.split(" ").collect();
  let mut max_row_len: Option<usize> = None; // a running tracker to make sure all rows are the same length

  for row in board_rows {
    let row_len = row.chars().collect::<Vec<char>>().len();

    match max_row_len {
      None => {
        // if max_row_len doesn't have a value, set it to the current row length.
        max_row_len = Some(row_len);
      }
      Some(i) => {
        // if max_row_len has a value, ensure that this new row we're checking is the same size as it. if it is, set max_row_len and proceed
        if i != row_len {
          println!(
            "rows must be of equal size! row \"{}\" is not the same length as the rest",
            row
          );
          exit(1);
        } else {
          max_row_len = Some(i);
        }
      }
    }
  }

  board_input
}

fn main() {
  let args: Vec<String> = args().collect();

  match args.len() {
    1 => {
      println!("No board argument provided!");
      exit(1);
    }
    2 => {
      if args[1].is_empty() {
        println!("No board argument provided!");
        exit(1);
      }
    }
    _ => (),
  }

  let board = validate_board(&args[1].trim());

  roggle::solve(board);
}
