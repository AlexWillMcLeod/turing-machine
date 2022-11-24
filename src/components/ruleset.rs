use super::Rule;

#[derive(PartialEq, Debug, Clone)]
pub struct RuleSet<Alphabet: Clone + Copy + PartialEq, States: PartialEq + Clone + Copy> {
  rules: Vec<Rule<Alphabet, States>>
}

impl<Alphabet: Clone + Copy + PartialEq, States: PartialEq + Copy> RuleSet<Alphabet, States> {
  pub fn new() -> RuleSet<Alphabet, States> {
    RuleSet {
      rules: Vec::<Rule::<Alphabet, States>>::new()
    }
  }
  pub fn add_rule(&mut self, new_rule: Rule<Alphabet, States>) {
    for rule in self.rules.iter() {
      if (*rule).apply_equally(&new_rule) {
        return;
      }
    }
    self.rules.push(new_rule);
  }
  pub fn remove_rule(&mut self, rule: Rule<Alphabet, States>) {
    let index = self.rules.iter().position(|x| *x == rule).unwrap();
    self.rules.swap_remove(index);
  }
  pub fn get_rule(&self, current_state: &States, current_value: &Alphabet) -> Option<&Rule<Alphabet, States>> {
    for rule in self.rules.iter() {
      if rule.applies(current_state, current_value) {
        return Some(rule);
      }
    }
    return None
  }
}

#[macro_export]
macro_rules! rules {
  (
    $($x: expr),*) => {{
    use turing_machine::components::{RuleSet, Rule};
    let mut ruleset = RuleSet::new();
    $(
      let new_rule = Rule::new($x.0, $x.1, $x.2, $x.3, $x.4);
      ruleset.add_rule(new_rule);
    )*
    ruleset
  }}
}