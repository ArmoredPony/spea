use objective::Objective;

pub mod breeder;
pub mod mutator;
pub mod objective;
pub mod selector;
pub mod terminator;

pub struct Spea2<T, const N: usize> {
  population: Vec<T>,
  archive: Vec<T>,
  objectives: [Box<dyn Objective<T>>; N],
}

impl<T, const N: usize> Spea2<T, N>
where
  T: Sync + Send,
{
  pub fn step(&mut self) {}
}
