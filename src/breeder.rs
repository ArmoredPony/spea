/// Responsible for creating offspring solutions by combining existing
/// solutions.
pub trait Breeder<T> {
  /// Combines genes of existing solutions and returns a generated set of
  /// solutions.
  fn breed(&mut self, population: &[T]) -> Vec<T>;
}

impl<T, F: Fn(&[T]) -> Vec<T>> Breeder<T> for F {
  fn breed(&mut self, population: &[T]) -> Vec<T> {
    self(population)
  }
}
