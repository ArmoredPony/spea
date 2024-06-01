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
  fn terminate(&mut self, population: &[T]) -> bool;
}

impl<T, F: Fn(&[T]) -> bool> Terminator<T> for F {
  fn terminate(&mut self, population: &[T]) -> bool {
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

/// Responsible for creating offspring solutions by combining existing
/// solutions.
pub trait Recombinator<T> {
  /// Combines genes of existing solutions and returns a generated set of
  /// solutions.
  fn combine(&mut self, population: &[T]) -> Vec<T>;
}

impl<T, F: Fn(&[T]) -> Vec<T>> Recombinator<T> for F {
  fn combine(&mut self, population: &[T]) -> Vec<T> {
    self(population)
  }
}

/// Represents an objective that solutions should converge on.
pub trait Objective<T, G> {
  /// Tests how close is current solution to the goal.
  fn test(&mut self, solution: &T) -> G;
}

impl<T, G, F: Fn(&T) -> G> Objective<T, G> for F {
  fn test(&mut self, solution: &T) -> G {
    self(solution)
  }
}
