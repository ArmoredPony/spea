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
pub struct Objectives<'a, S>(pub Vec<Box<dyn Objective<S> + 'a>>);

impl<'a, S> Objectives<'a, S> {
  /// Tests each objective and returns vector of resulting scores.
  pub fn test_each(&mut self, solution: &S) -> Vec<f32> {
    self.0.iter_mut().map(|o| o.test(solution)).collect()
  }
}

impl<'a, T, S, const N: usize> From<[T; N]> for Objectives<'a, S>
where
  T: Objective<S> + 'a,
{
  fn from(value: [T; N]) -> Self {
    Objectives(
      value
        .into_iter()
        .map(|v| Box::new(v) as Box<dyn Objective<S>>)
        .collect::<Vec<_>>(),
    )
  }
}
