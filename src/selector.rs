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
