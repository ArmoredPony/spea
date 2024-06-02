use std::cmp::Ordering;

/// Represents an objective that solutions should converge on.
pub trait Objective<T> {
  /// Tests how close is current solution to the goal.
  fn test(&self, solution: &T) -> f32;
}

impl<T, F: Fn(&T) -> f32> Objective<T> for F {
  fn test(&self, solution: &T) -> f32 {
    self(solution)
  }
}

struct Objectives<T, const N: usize>([Box<dyn Objective<T>>; N]);

trait ParetoDominance {
  fn dominance_ord(&self, other: &Self) -> Ordering;
}

impl ParetoDominance for &[f32] {
  fn dominance_ord(&self, other: &Self) -> Ordering {
    let mut ord = Ordering::Equal;
    for (s, o) in self.iter().zip(other.iter()) {
      let ord_i = s.partial_cmp(o).expect("attempted to compare a NaN");
      match (ord_i, ord) {
        (Ordering::Greater, Ordering::Less)
        | (Ordering::Less, Ordering::Greater) => return Ordering::Equal,
        (_, Ordering::Equal) => ord = ord_i,
        _ => (),
      }
    }
    ord
  }
}
