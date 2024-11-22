#[derive(Debug)]
pub struct Queue<T> {
  elements: Vec<T>,
}

impl<T> Queue<T> {
  pub fn new() -> Self {
    Queue {
      elements: Vec::new(),
    }
  }

  pub fn enqueue(&mut self, item: T) {
    self.elements.push(item);
  }

  pub fn dequeue(&mut self) -> Option<T> {
    if self.is_empty() {
      None
    } else {
      Some(self.elements.remove(0))
    }
  }

  pub fn is_empty(&self) -> bool {
    self.elements.is_empty()
  }

  pub fn size(&self) -> usize {
    self.elements.len()
  }

  pub fn peek(&self) -> Option<&T> {
    self.elements.first()
  }
}

impl<T> Default for Queue<T> {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::Queue;

  #[test]
  fn test_enqueue() {
    let mut queue = Queue::new();
    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);

    assert_eq!(queue.size(), 3);
    assert_eq!(queue.peek(), Some(&10));
  }

  #[test]
  fn test_dequeue() {
    let mut queue = Queue::new();
    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);

    assert_eq!(queue.dequeue(), Some(10));
    assert_eq!(queue.dequeue(), Some(20));
    assert_eq!(queue.size(), 1);
    assert_eq!(queue.peek(), Some(&30));
  }

  #[test]
  fn test_is_empty() {
    let mut queue = Queue::new();

    assert!(queue.is_empty());

    queue.enqueue(1);
    assert!(!queue.is_empty());
  }

  #[test]
  fn test_size() {
    let mut queue = Queue::new();
    assert_eq!(queue.size(), 0);

    queue.enqueue(42);
    assert_eq!(queue.size(), 1);

    queue.enqueue(100);
    assert_eq!(queue.size(), 2);

    queue.dequeue();
    assert_eq!(queue.size(), 1);
  }

  #[test]
  fn test_peek() {
    let mut queue = Queue::new();
    assert_eq!(queue.peek(), None);

    queue.enqueue(15);
    assert_eq!(queue.peek(), Some(&15));

    queue.enqueue(25);
    assert_eq!(queue.peek(), Some(&15));

    queue.dequeue();
    assert_eq!(queue.peek(), Some(&25));
  }

  #[test]
  fn test_dequeue_empty() {
    let mut queue: Queue<i32> = Queue::new();
    assert_eq!(queue.dequeue(), None);
  }

  #[test]
  fn test_trait_default() {
    let mut queue: Queue<i32>= Default::default();

    assert!(queue.is_empty());
    assert_eq!(queue.size(), 0);

    queue.enqueue(42);
    queue.enqueue(100);
    
    assert_eq!(queue.size(), 2);
    assert_eq!(queue.peek(), Some(&42));

    assert_eq!(queue.dequeue(), Some(42));
    assert_eq!(queue.peek(), Some(&100));
    assert_eq!(queue.dequeue(), Some(100));
    assert!(queue.is_empty());
  }
}

