/// Responsible for selecting solutions for breeding stage.
pub trait Selector<T> {
  /// Selects solutions from given population to procreate the next generation
  /// from.
  fn select<'a>(&'a mut self, population: &'a [T]) -> &'a [T];
}

impl<T, F: Fn(&[T]) -> &[T]> Selector<T> for F {
  fn select<'a>(&'a mut self, population: &'a [T]) -> &[T] {
    self(population)
  }
}
