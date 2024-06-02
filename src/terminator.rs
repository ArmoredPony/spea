/// Responsible for stopping genetic algorithm loop.
pub trait Terminator<T> {
  /// Returns `true` if termination condition is met.
  fn terminate(&mut self, population: &[T]) -> bool;
}

impl<T, F: Fn(&[T]) -> bool> Terminator<T> for F {
  fn terminate(&mut self, population: &[T]) -> bool {
    self(population)
  }
}
