use std::fmt::Debug;
use super::{RuleSet, TapeHead, Direction};

#[derive(Debug, Clone)]
pub struct TuringMachine<Alphabet: Clone + Copy + PartialEq, States: PartialEq + Copy> {
  tape: TapeHead<Alphabet>,
  current_state: States,
  accepting_states: Vec<States>,
  rules: RuleSet<Alphabet, States>,
}

impl<Alphabet: Clone + Copy + PartialEq + Debug, States: PartialEq + Clone + Copy> TuringMachine<Alphabet, States> {
  pub fn new(rules: RuleSet<Alphabet, States>, current_state: States, accepting_states: Vec<States>, initial_tape: Vec<Alphabet>, blank_value: Alphabet) -> Self {
    TuringMachine { 
      tape: TapeHead::new(initial_tape, blank_value), 
      current_state, 
      accepting_states, 
      rules
    }
  }
  pub fn get_state(&self) -> &States {
    &self.current_state
  }
  pub fn get_value(&self) -> Alphabet {
    self.tape.get_value()
  }
  pub fn get_tape(&self) -> &TapeHead<Alphabet> {
    &self.tape
  }
  fn set_state(&mut self, new_state: States) {
    self.current_state = new_state;
  }
  fn update_value(&mut self, direction: Option<Direction>, new_value: Alphabet) {
    self.tape.set_value(new_value);
    if let Some(new_direction) = direction {
      match new_direction {
        Direction::Left => self.tape.move_left(),
        Direction::Right => self.tape.move_right()
      }
    };
  }
  fn is_accepted(&self) -> bool {
    self.accepting_states.contains(self.get_state())
  }
}

impl<Alphabet: Clone + Copy + PartialEq + Debug, States: Clone + Copy + PartialEq> Iterator for TuringMachine<Alphabet, States> {

  type Item = TuringMachine<Alphabet, States>;

  fn next(&mut self) -> Option<Self::Item>{

    if self.is_accepted() {
      return None;
    }

    let applied_rule = self.rules.get_rule(self.get_state(), &self.get_value());
    
    if let Some(rule) = applied_rule {
      let (&next_value, &direction, &next_state) = rule.get_next();

      self.set_state(next_state);
      self.update_value(direction, next_value);

      return Some((*self).clone())
    } else {
      return None
    }

  }

}
