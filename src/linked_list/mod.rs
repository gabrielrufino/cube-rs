type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  value: T,
  next: Link<T>
}

pub struct LinkedList<T> {
  head: Link<T>
}

impl<T> LinkedList<T> {
  pub fn new() -> Self {
    LinkedList { head: None }
  }

  pub fn push_back(&mut self, value: T) {
    let new_node: Link<T> = Some(
      Box::new(Node {
        value,
        next: None,
      })
    );

    match self.head {
      None => {
        self.head = new_node
      }
      Some(ref mut head) => {
        let mut tail = head;

        while let Some(ref mut next) = tail.next {
          tail = next;
        }

        tail.next = new_node;
      }
    }
  }

  pub fn get_value_at(&self, index: usize) -> Option<&T> {
    let mut current = &self.head;
    let mut count: usize = 0;

    while let Some(ref node) = current {
      if count == index {
        return Some(&node.value)
      }

      current = &node.next;
      count += 1;
    }

    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_push_back_on_empty_list() {
    let mut list = LinkedList::new();
    list.push_back(1);
    assert_eq!(list.head.as_ref().unwrap().value, 1);
    assert!(list.head.as_ref().unwrap().next.is_none());
  }

  #[test]
  fn test_push_back_on_non_empty_list() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    assert_eq!(list.head.as_ref().unwrap().value, 1);
    assert_eq!(list.head.as_ref().unwrap().next.as_ref().unwrap().value, 2);
    assert!(list.head.as_ref().unwrap().next.as_ref().unwrap().next.is_none());
  }

  #[test]
  fn test_push_back_multiple_elements() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.head.as_ref().unwrap().value, 1);
    assert_eq!(list.head.as_ref().unwrap().next.as_ref().unwrap().value, 2);
    assert_eq!(list.head.as_ref().unwrap().next.as_ref().unwrap().next.as_ref().unwrap().value, 3);
    assert!(list.head.as_ref().unwrap().next.as_ref().unwrap().next.as_ref().unwrap().next.is_none());
  }

  #[test]
  fn test_get_value_at() {
    let mut list = LinkedList::new();
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    assert_eq!(list.get_value_at(0), Some(&10));
    assert_eq!(list.get_value_at(1), Some(&20));
    assert_eq!(list.get_value_at(2), Some(&30));

    assert_eq!(list.get_value_at(3), None);
  }
}
