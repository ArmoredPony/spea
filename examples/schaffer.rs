use itertools::Itertools;
use rand::{self, seq::SliceRandom, Rng};
use rand_distr::Distribution;
use spea::Spea2;

fn main() {
  let mut generations = 2000;
  let terminator = |_: &[&f32]| {
    generations -= 1;
    generations == 0
  };

  fn selector<'a>(v: &[&'a f32]) -> Vec<&'a f32> {
    v.choose_multiple(&mut rand::thread_rng(), 10)
      .copied()
      .collect()
  }

  let crossover = |population: &[&f32]| {
    population
      .iter()
      .cartesian_product(population)
      .map(|(a, b)| {
        let r = rand::thread_rng().gen_range(-0.25..1.25);
        *a * r + *b * (1.0 - r)
      })
      .collect()
  };

  let norm_dist = rand_distr::Normal::new(0.0f32, 200.0 / 6.0).unwrap();
  let mutator = |v: &mut f32| {
    if rand::thread_rng().gen::<f32>() <= 0.05 {
      *v += norm_dist.sample(&mut rand::thread_rng())
    }
  };

  let objectives = [
    |x: &f32| x.powf(2.0), //
    |x: &f32| (x - 2.0).powf(2.0),
  ];

  let initial_population = (-100..100).map(|v| v as f32).collect::<Vec<_>>();
  let archive_size = 100;

  let mut spea2 = Spea2::new(
    initial_population,
    archive_size,
    terminator,
    selector,
    crossover,
    mutator,
    objectives.into(),
  );

  spea2.run();
  let mut results = spea2.get_nondominated_solutions();
  results.sort_unstable_by(|a, b| a.total_cmp(b));
  println!("{:?}: {}", results, results.len());
}
