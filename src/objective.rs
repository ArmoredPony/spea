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
