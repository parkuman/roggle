use std::collections::HashSet;

const NEIGHBOURS: [(usize, usize); 8] = [
  (-1, -1),
  (0, -1),
  (1, -1),
  (-1, 0),
  (1, 0),
  (-1, 1),
  (0, 1),
  (1, 1),
];

fn find_words(board: &Board, x: usize, y: usize) -> HashSet<&'static str> {
  let found_words: HashSet<&str> = HashSet::new();

  for (n_x, n_y) in NEIGHBOURS.iter() {
    within_board(board, (x + n_x, y + n_y));
  }

  return found_words;
}

fn within_board(board: &Board, idx: (usize, usize)) -> bool {
  let (x, y) = idx;

  return if x < 0 || x >= board.width || y < 0 || y >= board.height {
    false
  } else {
    true
  };
}

pub struct Board {
  values: Vec<Vec<char>>,
  width: usize,
  height: usize,
}

impl Board {
  pub fn from(board_string: &str) -> Board {
    let rows = board_string.split(" ").collect::<Vec<&str>>();
    let mut values: Vec<Vec<char>> = vec![vec![char::default(); rows[0].len()]; rows.len()];

    for i in 0..rows.len() {
      let row = rows[i];
      for j in 0..row.chars().collect::<Vec<char>>().len() {
        values[i][j] = row.chars().nth(j).unwrap_or_default();
      }
    }

    return Board {
      values: values,
      width: rows[0].len(),
      height: rows.len(),
    };
  }

  pub fn get(&self, idx: (usize, usize)) -> char {
    self.values[idx.0][idx.1]
  }

  pub fn print(&self) {
    println!("values:");
    for row in self.values.iter() {
      for ch in row.iter() {
        print!("\t{} ", ch);
      }
      println!("\n");
    }

    println!("\nwidth: {}\nheight: {}", self.width, self.height);
  }

  pub fn solve(&self) -> HashSet<&str> {
    println!("\nSOLVING...");
    let mut words: HashSet<&str> = HashSet::new();

    println!("\nin board: {}", within_board(self, (0 - 1, 0 - 1)));
    println!("\nin board: {}", within_board(self, (0, 0)));
    println!("\nin board: {}", within_board(self, (3, 0)));
    println!("\nin board: {}", within_board(self, (4, 0)));

    // for y in 0..self.height {
    //   for x in 0..self.width {
    //     words.extend(find_words(self, x, y));
    //   }
    // }

    return words;
  }
}
