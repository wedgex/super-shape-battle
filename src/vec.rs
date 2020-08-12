pub fn intersection<T: std::cmp::PartialEq>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
  let mut intersection: Vec<T> = vec![];

  for item in vec1.into_iter() {
    if vec2.contains(&item) {
      intersection.push(item);
    }
  }

  intersection
}

pub fn intersections<T: std::cmp::PartialEq>(mut sets: Vec<Vec<T>>) -> Vec<T> {
  match (sets.pop(), sets.pop()) {
    (Some(set1), Some(set2)) => {
      sets.push(intersection(set1, set2));
      return intersections(sets);
    }
    (Some(set1), None) => return set1,
    _ => return vec![],
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn intersection_empty_for_empty_vecs() {
    let v1: Vec<&str> = vec![];
    let v2: Vec<&str> = vec![];

    assert_eq!(intersection(v1, v2).len(), 0);
  }

  #[test]
  fn intersection_empty_without_overlap() {
    let v1: Vec<&str> = vec!["1", "2"];
    let v2: Vec<&str> = vec!["4", "3"];

    let result = intersection(v1, v2);

    assert_eq!(result.len(), 0);
  }

  #[test]
  fn intersection_with_overlap() {
    let v1: Vec<&str> = vec!["1", "3", "2"];
    let v2: Vec<&str> = vec!["2", "4", "3"];

    let result = intersection(v1, v2);

    assert_eq!(result.len(), 2);
    assert_eq!(result, vec!["3", "2"]);
  }

  #[test]
  fn intersections_with_no_overlap() {
    let sets = vec![vec!["1", "2"], vec!["3", "4"], vec!["5", "6"]];

    let result = intersections(sets);

    assert_eq!(result.len(), 0);
  }

  #[test]
  fn intersections_with_overlap() {
    let sets = vec![
      vec!["1", "2", "5"],
      vec!["3", "1", "4", "5"],
      vec!["5", "6", "1"],
    ];

    let result = intersections(sets);

    assert_eq!(result.len(), 2);
    assert_eq!(result, vec!["5", "1"]);
  }
}
