/// Responsible for mutation of solutions in order to maintain genetic
/// diversity of a population.
pub trait Mutator<T> {
  /// Consumes a solution and returns a mutated solution.
  fn mutate(&mut self, solution: T) -> T;
}

impl<T, F: Fn(T) -> T> Mutator<T> for F {
  fn mutate(&mut self, solution: T) -> T {
    self(solution)
  }
}
