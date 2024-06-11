/// Responsible for creating offspring solutions by combining existing
/// solutions.
pub trait Breeder<T> {
  /// Combines genes of existing solutions and returns a generated set of
  /// solutions.
  fn breed(&self, population: &[&T]) -> Vec<T>;
}

impl<T, F: Fn(&[&T]) -> Vec<T>> Breeder<T> for F {
  fn breed(&self, population: &[&T]) -> Vec<T> {
    self(population)
  }
}

pub trait BinaryCrossover<T> {
  fn recombine(&self, a: &T, b: &T) -> T;
}

impl<T> Breeder<T> for dyn BinaryCrossover<T> {
  fn breed(&self, population: &[&T]) -> Vec<T> {
    population[..population.len() - 1]
      .iter()
      .enumerate()
      .flat_map(move |(i, a)| {
        population[i..].iter().map(|b| self.recombine(a, b))
      })
      .collect()
  }
}

impl<T, F: Fn(&T, &T) -> T> BinaryCrossover<T> for F {
  fn recombine(&self, a: &T, b: &T) -> T {
    self(a, b)
  }
}
