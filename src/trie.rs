use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
  value: Option<char>,
  is_word: bool,
  children: HashMap<char, TrieNode>,
}

impl TrieNode {
  pub fn create(c: char, is_word: bool) -> TrieNode {
    TrieNode {
      value: Some(c),
      is_word,
      children: HashMap::new(),
    }
  }

  pub fn create_root() -> TrieNode {
    TrieNode {
      value: None,
      is_word: false,
      children: HashMap::new(),
    }
  }

  pub fn check_value(self, c: char) -> bool {
    self.value == Some(c)
  }

  pub fn insert_value(&mut self, c: char, is_word: bool) {
    self.children.insert(c, TrieNode::create(c, is_word));
  }
}

#[derive(Debug)]
pub struct Trie {
  root_node: TrieNode,
}

impl Trie {
  pub fn new() -> Trie {
    Trie {
      root_node: TrieNode::create_root(),
    }
  }

  pub fn from(word_list: Vec<String>) -> Trie {
    let mut trie = Trie::new();

    print!("Building Trie... ");
    for word in word_list.iter() {
      // println!("Inserting {}...", word);
      trie.insert(word);
    }
    println!("Done!");

    return trie;
  }

  pub fn insert(&mut self, value: &str) {
    let chars: Vec<char> = value.chars().collect();
    let mut current_node = &mut self.root_node;
    let mut last_match_index = 0;

    for i in 0..chars.len() {
      if current_node.children.contains_key(&chars[i]) {
        // if one of the children contains the character we're looking for,
        //  then we need to set the current node to the one with the character, and
        // move to the next iteration (search that node's children for the next char)
        current_node = current_node.children.get_mut(&chars[i]).unwrap();
        last_match_index = i + 1; // since we have a match, set the last_match_index to the next character after this one we matched
      } else {
        // if the char isn't one of the children of a node, set the last matched index to the current char counter and break out of the loop
        last_match_index = i;
        break;
      }
    }

    // if we finished the whole word, set the "is_word" bool for the node we stopped on to true
    if last_match_index == chars.len() {
      current_node.is_word = true;

    // if we didn't finish the whole word, insert the rest of the characters and set the last node's "is_word" to true
    } else {
      for i in last_match_index..chars.len() {
        // println!("{} -> {}", current_node.value.unwrap_or_default(), chars[i]);
        current_node.insert_value(chars[i], false);
        current_node = current_node.children.get_mut(&chars[i]).unwrap(); // increment to the next node (the new one we just inserted)
      }

      current_node.is_word = true;
    }
  }

  pub fn find(&mut self, value: &str, is_word: &mut bool) -> bool {
    let chars: Vec<char> = value.chars().collect();
    let mut current_node = &mut self.root_node;

    for i in 0..chars.len() {
      if !current_node.children.contains_key(&chars[i]) {
        return false; // if the node's children don't contain the char we need, then it can't possibly contain the whole word
      } else {
        current_node = current_node.children.get_mut(&chars[i]).unwrap();
      }
    }

    if current_node.is_word {
      *is_word = true;
    }

    // if we've gone through the Trie and found every character of the word, then the Trie contains the word
    return true;
  }
}
