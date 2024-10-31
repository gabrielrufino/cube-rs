#[derive(Debug)]
pub struct Stack<T> {
  elements: Vec<T>,
}

impl<T> Stack<T> {
  pub fn new() -> Self {
    Stack {
      elements: Vec::new()
    }
  }

  pub fn push(&mut self, item: T) {
    self.elements.push(item)
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.is_empty() {
      None
    } else {
      self.elements.pop()
    }
  }

  pub fn is_empty(&self) -> bool {
    self.elements.is_empty()
  }

  pub fn size(&self) -> usize {
    self.elements.len()
  }

  pub fn peek(&self) -> Option<&T> {
    self.elements.last()
  }
}

impl<T> Default for Stack<T> {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::Stack;

  #[test]
  fn test_push() {
    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);

    assert_eq!(stack.size(), 3);
    assert_eq!(stack.peek(), Some(&30));
  }

  #[test]
  fn test_pop() {
    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);

    assert_eq!(stack.pop(), Some(30));
    assert_eq!(stack.pop(), Some(20));
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.peek(), Some(&10));
  }

  #[test]
  fn test_pop_empty() {
    let mut stack: Stack<i32> = Stack::new();
    assert_eq!(stack.pop(), None);
  }

  #[test]
  fn test_size() {
    let mut stack = Stack::new();
    assert_eq!(stack.size(), 0);

    stack.push(42);
    assert_eq!(stack.size(), 1);

    stack.push(100);
    assert_eq!(stack.size(), 2);

    stack.pop();
    assert_eq!(stack.size(), 1);
  }

  #[test]
  fn test_peek() {
    let mut stack = Stack::new();
    assert_eq!(stack.peek(), None);

    stack.push(15);
    assert_eq!(stack.peek(), Some(&15));

    stack.push(25);
    assert_eq!(stack.peek(), Some(&25));

    stack.pop();
    assert_eq!(stack.peek(), Some(&15));
  }

  #[test]
  fn test_trait_default() {
    let mut stack: Stack<i32>= Default::default();

    assert!(stack.is_empty());
    assert_eq!(stack.size(), 0);

    stack.push(42);
    stack.push(100);
    
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.peek(), Some(&100));

    assert_eq!(stack.pop(), Some(100));
    assert_eq!(stack.peek(), Some(&42));
    assert_eq!(stack.pop(), Some(42));
    assert!(stack.is_empty());
  }
}
