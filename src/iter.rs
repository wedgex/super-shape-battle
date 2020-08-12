use std::collections::HashSet;
use std::hash::Hash;

struct Unique<I>
where
  I: Iterator,
  I::Item: Hash + Eq + Clone,
{
  iter: I,
  used: HashSet<I::Item>,
}

impl<I: Iterator> Iterator for Unique<I>
where
  I::Item: Hash + Eq + Clone,
{
  type Item = I::Item;

  fn next(&mut self) -> Option<<Self as std::iter::Iterator>::Item> {
    while let Some(next) = self.iter.next() {
      if self.used.insert(next.clone()) {
        return Some(next);
      }
    }
    None
  }
}

trait UniqueIter: Iterator {
  fn unique(self) -> Unique<Self>
  where
    Self: Sized,
    Self::Item: Eq + Hash + Clone,
  {
    Unique {
      iter: self,
      used: HashSet::new(),
    }
  }
}

impl<I: Iterator> UniqueIter for I {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn unique_when_empty() {
    let items: Vec<String> = vec![];
    let mut unique = items.into_iter().unique();

    assert!(unique.next().is_none());
  }

  #[test]
  fn unique_without_duplicates() {
    let items = vec!["one", "two", "three"];
    let mut unique = items.into_iter().unique();

    assert_eq!(unique.next().unwrap().to_string(), String::from("one"));
    assert_eq!(unique.next().unwrap().to_string(), String::from("two"));
    assert_eq!(unique.next().unwrap().to_string(), String::from("three"));
    assert!(unique.next().is_none());
  }

  #[test]
  fn unique_with_duplicates() {
    let items = vec!["one", "two", "one", "three", "two"];
    let mut unique = items.into_iter().unique();

    assert_eq!(unique.next().unwrap().to_string(), String::from("one"));
    assert_eq!(unique.next().unwrap().to_string(), String::from("two"));
    assert_eq!(unique.next().unwrap().to_string(), String::from("three"));
    assert!(unique.next().is_none());
  }
}
