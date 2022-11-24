use super::Direction;
use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Rule<Alphabet: Clone + Copy, States: Clone + Copy> {
  current_state: States,
  current_value: Alphabet,
  next_symbol: Alphabet,
  direction: Option<Direction>,
  next_state: States
}

impl<Alphabet: Clone + Copy + PartialEq, States: PartialEq + Copy> Rule<Alphabet, States> {
  pub fn new(current_state: States, current_value: Alphabet, next_symbol: Alphabet, direction: Option<Direction>, next_state: States) -> Self {
    Rule {
      current_state,
      current_value,
      next_symbol,
      direction,
      next_state
    }
  }
  pub fn applies (&self, current_state: &States, current_value: &Alphabet) -> bool {
    self.current_state == *current_state &&
    self.current_value == *current_value
  }
  pub fn get_next(&self) -> (&Alphabet, &Option<Direction>, &States) {
    (&self.next_symbol, &self.direction, &self.next_state)
  }
  pub fn apply_equally(&self, other: &Self) -> bool {
    self.current_state == other.current_state &&
    self.current_value == other.current_value
  }
}