/// Responsible for stopping genetic algorithm loop.
pub trait Terminator<T> {
  /// Returns `true` if termination condition is met.
  fn terminate(&mut self, population: &[(T, f32)]) -> bool;
}

impl<T, F: FnMut(&[(T, f32)]) -> bool> Terminator<T> for F {
  fn terminate(&mut self, population: &[(T, f32)]) -> bool {
    self(population)
  }
}

/// Terminates algorithm after a certain number of generations.
pub struct Generations {
  generations: u32,
}

impl Generations {
  /// Creates `Generations` terminator.
  pub fn new(generations: u32) -> Self {
    Generations { generations }
  }
}

impl<T> Terminator<T> for Generations {
  fn terminate(&mut self, _: &[(T, f32)]) -> bool {
    match self.generations {
      0 => true,
      _ => {
        self.generations -= 1;
        false
      }
    }
  }
}
