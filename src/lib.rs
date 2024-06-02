pub mod breeder;
pub mod mutator;
pub mod objective;
pub mod selector;
pub mod terminator;

pub struct Spea2<T> {
  population: Vec<T>,
  archive: Vec<T>,
}

impl<T> Spea2<T>
where
  T: Sync + Send,
{
  pub fn step(&mut self) {}
}
