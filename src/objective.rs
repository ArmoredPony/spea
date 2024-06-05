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
