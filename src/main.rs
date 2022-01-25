use std::{
  fs::File,
  io::{BufRead, BufReader, Result},
  collections::{HashSet as Set, HashMap}
};

fn load_dict() -> Result<Set<String>> {
  let file = File::open("wordle_dict.txt").expect("");
  BufReader::new(file)
    .lines()
    .collect()
}

enum LetterResult {
  Green(Set<usize>),
  Yellow(Set<usize>),
  Gray
}

fn check(guess: &str, actual: &str) -> HashMap<char, LetterResult> {
  guess.chars().enumerate()
    .map(|(i, c)| { 
      (c, match actual.find(c) {
        Some  (n ) if n == i => {
          let mut set = Set::new();
          set.insert(n);
          LetterResult::Green(set)
        }
        Some (n) => {
          let mut set = Set::new();
          set.insert(n);
          LetterResult::Yellow(set)
        },
        None => LetterResult::Gray
      })
    })
    .collect()
}

fn matches_constraints(constraints: &HashMap<char, LetterResult>, word: &str) -> bool {
  constraints.iter().all(|(&c, lr)| matches_constraint(word, c, lr))
}

fn matches_constraint(word: &str, c: char, lr: &LetterResult) -> bool {
  match lr {
    LetterResult::Green(idxs) => idxs.iter().all(|&i| word.chars().nth(i) == Some(c)),
    LetterResult::Yellow(not_idxs) => word.contains(c) && not_idxs.iter().all(|&i| word.chars().nth(i) != Some(c)),
    LetterResult::Gray => !word.contains(c),
  }
}

fn main() -> Result<()> {
  let wordle_dict = load_dict()?;

  // for each guess in dictionary,
  // group possible secret words in dictionary into buckets based on result,
  // and take max bucket size
  let guess = wordle_dict.iter()
    .min_by_key(|guess| {
      let biggest_bucket = 
        wordle_dict.iter()
        .map(|actual| {
          let constraints = check(guess, actual);
          wordle_dict.iter().filter(|w| matches_constraints(&constraints, w)).count()
        }).max().unwrap();
      println!("Worst case for {}: {} candidates", guess, biggest_bucket);
      biggest_bucket
    });

  println!("{:?}", guess);
  Ok(())
}
