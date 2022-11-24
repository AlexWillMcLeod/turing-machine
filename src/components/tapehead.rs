use std::{cell::RefCell, rc::Rc};
use std::fmt::{self, Display, Debug};

struct Bit<T: Clone + Copy> {
  value: Option<T>,
  left: Option<Rc<RefCell<Bit<T>>>>,
  right: Option<Rc<RefCell<Bit<T>>>>
}

#[derive(Clone)]
pub struct TapeHead<T: Clone + Copy> {
  ptr: Rc<RefCell<Bit<T>>>,
  default_value: T
}

impl<T: Clone + Copy> TapeHead<T> {
  pub fn new(initial_tape: Vec::<T>, default_value: T) -> TapeHead<T> {
    let mut new_tape_head = TapeHead {
      ptr: Rc::new(RefCell::new(Bit {
        value: None,
        left: None,
        right: None
      })),
      default_value
    };
    for &element in initial_tape.iter() {
      new_tape_head.set_value(element);
      new_tape_head.move_right();
    }
    for _ in 0..initial_tape.len() {
      new_tape_head.move_left();
    }
    new_tape_head
  }
  pub fn get_value(&self) -> T {
    match self.ptr.borrow().value {
      Some(value) => value.clone(),
      None => self.default_value
    }
  }
  pub fn set_value(&mut self, new_value: T) {
    self.ptr.borrow_mut().value = Some(new_value)
  }
  pub fn move_left(&mut self) {

    let new_left = match &self.ptr.borrow().left {
      Some(item) => Rc::clone(&item),
      None => {    
        Rc::new(RefCell::new(Bit {
          value: None,
          right: Some(Rc::clone(&self.ptr)),
          left: None
        }))
      }
    };

    self.ptr.borrow_mut().left = Some(Rc::clone(&new_left));
    self.ptr = Rc::clone(&new_left);

  }
  pub fn move_right(&mut self) {

    let new_right = match &self.ptr.borrow().right {
      Some(item) => Rc::clone(&item),
      None => {    
        Rc::new(RefCell::new(Bit {
          value: None,
          left: Some(Rc::clone(&self.ptr)),
          right: None
        }))
      }
    };

    self.ptr.borrow_mut().right = Some(Rc::clone(&new_right));
    self.ptr = Rc::clone(&new_right);

  }
  pub fn is_left(&self) -> bool{
    match &self.ptr.borrow().left {
      Some(..) => true,
      None => false
    }
  }
  pub fn is_right(&self) -> bool{
    match &self.ptr.borrow().right {
      Some(..) => true,
      None => false
    }
  }
}

impl<T: Clone + Copy + Debug> Display for TapeHead<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "..., ").unwrap();
    // Clone Tapehead to not affect current tapehead.
    let mut cloned_tapehead = self.clone();
    let mut left_from_ptr = 0;
    // Go as far left as possible
    (cloned_tapehead).move_left();
    left_from_ptr += 1;
    while cloned_tapehead.is_left() {
      (cloned_tapehead).move_left();
      left_from_ptr += 1;
    };
    // Starting counting every element and adding to our array
    if left_from_ptr == 0 {
      write!(f, "->{:?}<-, ", cloned_tapehead.get_value()).unwrap();
    } else {
      write!(f, "{:?}, ", cloned_tapehead.get_value()).unwrap();
    }
    while cloned_tapehead.is_right() {
      cloned_tapehead.move_right();
      left_from_ptr -= 1;
      if left_from_ptr == 0 {
        write!(f, "->{:?}<-, ", cloned_tapehead.get_value()).unwrap();
      } else {
        write!(f, "{:?}, ", cloned_tapehead.get_value()).unwrap();
      }
    };
    write!(f, "...").unwrap();
    Ok(())
  }
}

impl<T: Copy + Debug> std::fmt::Debug for TapeHead<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "..., ").unwrap();
    // Clone Tapehead to not affect current tapehead.
    let mut cloned_tapehead = self.clone();
    let mut left_from_ptr = 0;
    // Go as far left as possible
    (cloned_tapehead).move_left();
    left_from_ptr += 1;
    while cloned_tapehead.is_left() {
      cloned_tapehead.move_left();
      left_from_ptr += 1;
    };
    // Starting counting every element and adding to our array
    if left_from_ptr == 0 {
      write!(f, "->{:?}<-, ", cloned_tapehead.get_value()).unwrap();
    } else {
      write!(f, "{:?}, ", cloned_tapehead.get_value()).unwrap();
    }
    while cloned_tapehead.is_right() {
      cloned_tapehead.move_right();
      left_from_ptr -= 1;
      if left_from_ptr == 0 {
        write!(f, "->{:?}<-, ", cloned_tapehead.get_value()).unwrap();
      } else {
        write!(f, "{:?}, ", cloned_tapehead.get_value()).unwrap();
      }
    };
    write!(f, "...").unwrap();
    Ok(())
  }
}