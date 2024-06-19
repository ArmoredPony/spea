/// Responsible for creating offspring solutions by combining existing
/// solutions.
pub trait Crossover<T> {
  /// Combines genes of existing solutions and returns a generated set of
  /// solutions.
  fn recombine(&self, population: &[&T]) -> Vec<T>;
}

impl<T, F> Crossover<T> for F
where
  F: Fn(&[&T]) -> Vec<T>,
{
  fn recombine(&self, population: &[&T]) -> Vec<T> {
    self(population)
  }
}
