/// Responsible for mutation of solutions in order to maintain genetic
/// diversity of a population.
pub trait Mutator<T> {
  /// Consumes a solution and returns a mutated solution.
  fn mutate(&self, solution: &mut T);
}

impl<T, F: Fn(&T)> Mutator<T> for F {
  fn mutate(&self, solution: &mut T) {
    self(solution)
  }
}
