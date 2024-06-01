use rayon::prelude::*;

/// Responsible for selecting solutions for breeding stage.
pub trait Selector<T> {
  /// Selects solutions from given population to procreate the next generation
  /// from.
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

/// Represents an objective that solutions should converge on.
pub trait Objective<T> {
  /// Tests how close is current solution to the goal.
  fn test(&self, solution: &T) -> f32;
}

impl<T, F: Fn(&T) -> f32> Objective<T> for F {
  fn test(&self, solution: &T) -> f32 {
    self(solution)
  }
}
