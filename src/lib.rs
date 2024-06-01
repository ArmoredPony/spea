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
