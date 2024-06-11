use rand::{self, Rng};
use spea::{
  breeder::{BinaryCrossover, Breeder},
  terminator::Generations,
  Spea2,
};

fn main() {
  fn crossover<'a>(a: &'a f32, b: &'a f32) -> f32 {
    let r = rand::thread_rng().gen_range(-0.25..1.25);
    a * r + b * (1.0 - r)
  }

  fn selector<'a>(v: &'a [(&'a f32, f32)]) -> Vec<&'a f32> {
    v.iter().take(10).map(|(s, _)| *s).collect()
  }

  let objectives = [|x: &f32| x.powf(2.0), |x: &f32| (x - 2.0).powf(2.0)];

  let initial_population = (0..10).map(|v| v as f32).collect::<Vec<_>>();
  let archive_size = initial_population.len();

  let spea2 = Spea2::new(
    initial_population,
    archive_size,
    Generations::new(100),
    selector,
    // FIXME: it should be replaced with a closure
    Crossover {},
    |_: &_| (),
    objectives.into(),
  );

  let results = spea2.run();
  println!("{:?}", results);
}

struct Crossover;

impl Breeder<f32> for Crossover {
  fn breed(&self, population: &[&f32]) -> Vec<f32> {
    let p = population[..population.len() - 1]
      .iter()
      .enumerate()
      .flat_map(move |(i, a)| {
        population[i..].iter().map(|b| {
          let r = rand::thread_rng().gen_range(-0.25..1.25);
          *a * r + *b * (1.0 - r)
        })
      })
      .collect::<Vec<_>>();
    println!("solutions after breeding: {}", p.len());
    p
  }
}
