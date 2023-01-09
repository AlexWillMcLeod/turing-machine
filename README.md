# Turing Machine in Rust
Implement a Turing Machine in rust. Initiate your alphabet (or use default), initiate states, and use the Turing machine as an iterator between the states.

```rust
use turing_machine::{DefaultAlphabet::{On, Off, self}, TuringMachine, Direction::{Left, Right}, rules};

  
  // Create your states as an enum with the following traits
  #[derive(PartialEq, Debug, Copy, Clone)]
  enum States { A, B, C } 
  use States::{A, B, C};

  
  // Use rules! macro to initialise your machines rules 
  
 let mut machine = TuringMachine::new(
   rules![
      (A, Off, On, Some(Right), B),
      (A, On, On, Some(Right), B),
      (B, Off, Off, Some(Right), A),
      (B, On, Off, Some(Right), A)
    ],
    B, // Initial State
    vec![C],
    vec![Off], // Starting Tape
    Off // Blank (default) value for the rest of tape
  );

  // Use machine as an iterator to iterate through the different states of the machine
  
  for value in machine.take(10) {
    println!("{:?}", machine); // Print the first ten states of the machine
  }
```
