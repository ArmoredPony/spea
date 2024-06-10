/// Responsible for selecting solutions for breeding stage.
pub trait Selector<T> {
  /// Selects solutions from given population to procreate the next generation
  /// from.
  fn select<'a>(&'a mut self, population: &'a [&'a T]) -> Vec<&'a T>;
}

impl<T, F: for<'a> Fn(&'a [&'a T]) -> Vec<&'a T>> Selector<T> for F {
  fn select<'a>(&'a mut self, population: &'a [&'a T]) -> Vec<&'a T> {
    self(population)
  }
}
