extern crate turing_machine;

use turing_machine::{DefaultAlphabet::{On, Off, self}, TuringMachine, Direction::{Left, Right}, rules};

fn main () {

  #[derive(PartialEq, Debug, Copy, Clone)]
  enum States { A, B, C }
  use States::{A, B, C};

  let mut machine = TuringMachine::new(
    rules![
      (A, Off, On, Some(Right), B),
      (A, On, On, Some(Right), B),
      (B, Off, Off, Some(Right), A),
      (B, On, Off, Some(Right), A)
    ],
    B,
    vec![C],
    vec![Off],
    Off
  );
  
  println!("{:#?}", *machine.get_tape());
  loop {
    let value = machine.next();
    println!("{:#?}", value.unwrap().get_tape());
  }


}