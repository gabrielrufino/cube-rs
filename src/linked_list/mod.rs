type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
  value: T,
  next: Link<T>
}

#[derive(Debug)]
pub struct LinkedList<T> {
  head: Link<T>
}

impl<T> LinkedList<T> {
  pub fn new() -> Self {
    LinkedList { head: None }
  }

  pub fn size(&self) -> usize {
    let mut count = 0;
    let mut current = &self.head;

    while let Some(node) = current {
      count += 1;
      current = &node.next;
    }

    count
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

  pub fn remove_at(&mut self, index: usize) -> Option<T> {
    match index {
      0 => {
        match self.head.take() {
          Some(mut head) => {
            self.head = head.next.take();
            Some(head.value)
          },
          None => None,
        }
      },
      _ => {
        let mut current = &mut self.head;
        let mut count = 0;

        while let Some(ref mut node) = current {
          match count + 1 == index {
            true => {
              match node.next.take() {
                Some(mut node_to_remove) => {
                  node.next = node_to_remove.next.take();
                  return Some(node_to_remove.value);
                },
                None => {
                  return None;
                },
              }
            },
            false => {
              current = &mut node.next;
              count += 1;
            }
          }
        }

        None
      }
    }
  }
}

impl<T> Default for LinkedList<T> {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod size {
    use super::*;

    #[test]
    fn test_size_empty_list() {
      let list: LinkedList<i32> = LinkedList::new();
      assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_size_single_element() {
      let mut list = LinkedList::new();
      list.push_back(1);
      assert_eq!(list.size(), 1);
    }

    #[test]
    fn test_size_multiple_elements() {
      let mut list = LinkedList::new();
      list.push_back(1);
      list.push_back(2);
      list.push_back(3);
      assert_eq!(list.size(), 3);
    }

    #[test]
    fn test_size_after_removal() {
      let mut list = LinkedList::new();
      list.push_back(1);
      list.push_back(2);
      list.push_back(3);
      list.remove_at(1);
      assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_size_after_clearing() {
      let mut list = LinkedList::new();
      list.push_back(1);
      list.push_back(2);
      list.push_back(3);

      list.remove_at(0);
      list.remove_at(0);
      list.remove_at(0);

      assert_eq!(list.size(), 0);
    }
  }

  mod push_back {
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
  }

  mod get_value_at {
    use super::*;

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

  mod remove_at {
    use super::*;

    #[test]
    fn test_remove_at_head() {
      let mut list = LinkedList::new();
      list.push_back(10);
      list.push_back(20);
      list.push_back(30);

      assert_eq!(list.remove_at(0), Some(10));
      assert_eq!(list.get_value_at(0), Some(&20));
      assert_eq!(list.get_value_at(1), Some(&30));
      assert_eq!(list.get_value_at(2), None);
    }
    
    #[test]
    fn test_remove_at_middle() {
      let mut list = LinkedList::new();
      list.push_back(10);
      list.push_back(20);
      list.push_back(30);

      assert_eq!(list.remove_at(1), Some(20));
      assert_eq!(list.get_value_at(0), Some(&10));
      assert_eq!(list.get_value_at(1), Some(&30));
      assert_eq!(list.get_value_at(2), None);
    }
    
    #[test]
    fn test_remove_at_end() {
      let mut list = LinkedList::new();
      list.push_back(10);
      list.push_back(20);
      list.push_back(30);

      assert_eq!(list.remove_at(2), Some(30));
      assert_eq!(list.get_value_at(0), Some(&10));
      assert_eq!(list.get_value_at(1), Some(&20));
      assert_eq!(list.get_value_at(2), None);
    }
    
    #[test]
    fn test_remove_invalid_index() {
      let mut list = LinkedList::new();
      list.push_back(10);
      list.push_back(20);
      list.push_back(30);

      assert_eq!(list.remove_at(3), None);
      assert_eq!(list.get_value_at(0), Some(&10));
      assert_eq!(list.get_value_at(1), Some(&20));
      assert_eq!(list.get_value_at(2), Some(&30));
    }

    #[test]
    fn test_remove_head_when_it_is_the_only_element() {
      let mut list = LinkedList::new();
      list.push_back(1);

      assert_eq!(list.remove_at(0), Some(1));
      assert!(list.head.is_none());
    }
  }
}
