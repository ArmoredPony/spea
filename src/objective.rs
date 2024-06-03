use std::cmp::Ordering;

/// Represents an objective that solutions should converge on.
pub trait Objective<S> {
  /// Tests how close is current solution to the goal.
  /// The target score of an objective is 0.
  /// Score can be negative, only its distance to 0 matters.
  fn test(&self, solution: &S) -> f32;
}

impl<S, F: Fn(&S) -> f32> Objective<S> for F {
  fn test(&self, solution: &S) -> f32 {
    self(solution)
  }
}

pub(super) struct Objectives<S>(pub Vec<Box<dyn Objective<S> + Send + Sync>>);

pub(super) trait ParetoDominance {
  /// Calculates pareto dominance ordering. Returns
  /// - `Less` if `self` dominates `other`
  /// - `Greater` if `other` dominates `self`
  /// - `Equal` otherwise
  fn pareto_dominance_ord(&self, other: &Self) -> Ordering;
}

impl ParetoDominance for &[f32] {
  fn pareto_dominance_ord(&self, other: &Self) -> Ordering {
    let mut ord = Ordering::Equal;
    for (s, o) in self.iter().zip(other.iter()) {
      let ord_i = s
        .abs()
        .partial_cmp(&o.abs())
        .expect("attempted to compare a NaN");
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
