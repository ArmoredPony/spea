/// Responsible for selecting solutions for breeding stage.
pub trait Selector<T> {
  /// Selects solutions from given population to procreate the next generation
  /// from. Solutions are passed in tuple with their calculated fitness values.
  /// Fitness values are calculated according to the SPEA2 algorithm. The
  /// smaller the better.
  fn select<'a>(&mut self, population: &[&'a T]) -> Vec<&'a T>;
}

impl<T, F: for<'a> Fn(&[&'a T]) -> Vec<&'a T>> Selector<T> for F {
  fn select<'a>(&mut self, population: &[&'a T]) -> Vec<&'a T> {
    self(population)
  }
}
