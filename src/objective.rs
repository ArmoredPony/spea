/// Represents an objective that solutions should converge on.
pub trait Objective<S> {
  /// Tests how close is current solution to the goal.
  /// The target score of an objective is 0.
  /// Score can be negative, only its distance to 0 matters.
  fn test(&mut self, solution: &S) -> f32;
}

impl<S, F: FnMut(&S) -> f32> Objective<S> for F {
  fn test(&mut self, solution: &S) -> f32 {
    self(solution)
  }
}

/// Containes boxed `Objective`s.
pub struct Objectives<'a, U>(pub Vec<Box<dyn Objective<U> + Send + Sync + 'a>>);

impl<'a, T, U, const N: usize> From<[T; N]> for Objectives<'a, U>
where
  T: Objective<U> + Send + Sync + 'a,
{
  fn from(value: [T; N]) -> Self {
    Objectives(
      value
        .into_iter()
        .map(|v| Box::new(v) as Box<dyn Objective<U> + Send + Sync>)
        .collect::<Vec<_>>(),
    )
  }
}
