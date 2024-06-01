/// Represents a selector used in selection stage.
pub trait Selector<T> {
  /// Selects solutions from given population for the next generation.
  fn select(&mut self, population: Vec<T>) -> Vec<T>;
}

impl<T, F: Fn(Vec<T>) -> Vec<T>> Selector<T> for F {
  fn select(&mut self, population: Vec<T>) -> Vec<T> {
    self(population)
  }
}

/// Responsible for stopping genetic algorithm loop.
pub trait Terminator<T> {
  /// Returns `true` if termination condition is met.
  fn terminate(&mut self, population: Vec<T>) -> bool;
}

impl<T, F: Fn(Vec<T>) -> bool> Terminator<T> for F {
  fn terminate(&mut self, population: Vec<T>) -> bool {
    self(population)
  }
}

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
