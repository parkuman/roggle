use crate::trie::Trie;

const NEIGHBOURS: [(i16, i16); 8] = [
  (-1, -1),
  (0, -1),
  (1, -1),
  (-1, 0),
  (1, 0),
  (-1, 1),
  (0, 1),
  (1, 1),
];

fn find_words<'a>(
  board: &Board,
  x: i16,
  y: i16,
  visited_positions: &mut Vec<(i16, i16)>,
  current_string: &mut String,
  trie: &mut Trie,
) -> Vec<String> {
  let mut found_words: Vec<String> = Vec::new();
  let mut is_word = false;

  // if the current working word is not in the trie, return early.
  // otherwise add our current working word to our found_words list
  if !trie.find(current_string, &mut is_word) {
    // println!(
    //   "❌ the prefix \"{}\" was not found in the trie, exiting this recursion",
    //   current_string
    // );
    current_string.pop(); // pop off the bad letter we added

    return found_words;
  }

  // println!("✅ the prefix \"{}\" is in the trie!", current_string);

  if is_word {
    // println!("\tAND its a full word!");
    found_words.push(current_string.to_string());
  }

  for (n_x, n_y) in NEIGHBOURS.iter() {
    let new_x = x + n_x;
    let new_y = y + n_y;

    // if the neighbour is within the board and hasn't already been visited, recurse, searching the neighbour for words
    if within_board(board, (new_x, new_y)) && !visited_positions.contains(&(new_x, new_y)) {
      current_string.push(board.get((new_x, new_y)));

      let last_visited_idx = visited_positions.len();
      visited_positions.push((new_x, new_y));

      found_words.extend(find_words(
        board,
        new_x,
        new_y,
        visited_positions,
        current_string, // TODO: PLUS ANOTHER CHAR
        trie,
      ));

      visited_positions.remove(last_visited_idx); // remove the added visited position
    }
  }

  current_string.pop();

  return found_words;
}

fn within_board(board: &Board, pos: (i16, i16)) -> bool {
  let (x, y) = pos;
  return if x < 0 || x >= board.width || y < 0 || y >= board.height {
    false
  } else {
    true
  };
}

pub struct Board {
  values: Vec<Vec<char>>,
  width: i16,
  height: i16,
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
      width: (rows[0].len() as i16),
      height: (rows.len() as i16),
    };
  }

  pub fn get(&self, pos: (i16, i16)) -> char {
    self.values[(pos.1 as usize)][(pos.0 as usize)]
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

  pub fn solve(&self, trie: &mut Trie) -> Vec<String> {
    println!("\nSOLVING...");
    let mut words: Vec<String> = Vec::new();

    // for x in 0..self.width {
    //   for y in 0..self.height {
    //     println!("========| ({}, {}) - {} |========", x, y, self.get((x, y)));

    //     for (n_x, n_y) in NEIGHBOURS.iter() {
    //       let new_x = x + n_x;
    //       let new_y = y + n_y;

    //       let in_board = within_board(self, (new_x, new_y));
    //       print!("({}, {}) : {}", new_x, new_y, in_board,);

    //       if in_board {
    //         print!(" - \"{}\"", self.get((new_x, new_y)));
    //       }

    //       println!();
    //     }
    //   }
    // }

    let mut visited_positions;
    let mut current_string;

    // hit each letter in the board and call find_words on it
    for x in 0..self.width {
      for y in 0..self.height {
        // reset the following for each new root letter we start at
        current_string = self.get((x, y)).to_string();
        visited_positions = Vec::new();

        words.extend(find_words(
          self,
          x,
          y,
          &mut visited_positions,
          &mut current_string,
          trie,
        ));
      }
    }

    return words;
  }
}
