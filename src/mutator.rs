/// Responsible for mutation of solutions in order to maintain genetic
/// diversity of a population.
pub trait Mutator<T> {
  /// Consumes a solution and returns a mutated solution.
  fn mutate(&mut self, solution: &mut T);
}

impl<T, F: FnMut(&mut T)> Mutator<T> for F {
  fn mutate(&mut self, solution: &mut T) {
    self(solution)
  }
}
