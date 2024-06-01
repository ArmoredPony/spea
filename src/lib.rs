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
