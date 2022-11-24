```
extern crate turing_machine;

use turing_machine::{DefaultAlphabet::{On, Off, self}, TuringMachine, Direction::{Left, Right}, rules};

fn main () {

  #[derive(PartialEq, Debug, Copy, Clone)]
  enum States { A, B, C } // Create your states with the following traits
  use States::{A, B, C};

  let mut machine = TuringMachine::new(
    rules![ // Use the rules macro to create rules
      (A, Off, On, Some(Right), B),
      (A, On, On, Some(Right), B),
      (B, Off, Off, Some(Right), A),
      (B, On, Off, Some(Right), A)
    ],
    B, // Initial State
    vec![C], // Accepting States
    vec![Off], // Starting Tape
    Off // Blank (default) value for the rest of tape
  );

  // Use machine as an iterator to iterate through the different states of the machine
  // Eg:

  for value in machine.take(10) {
    println!("{:?}", machine); // Print the first ten states of the machine
  }


}
```
