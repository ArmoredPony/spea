use std::cmp::Ordering;

use breeder::Breeder;
use mutator::Mutator;
use objective::Objective;
use rayon::prelude::*;
use selector::Selector;
use terminator::Terminator;

pub mod breeder;
pub mod mutator;
pub mod objective;
pub mod selector;
pub mod terminator;

pub struct Spea2<'a, U, T, S, B, M>
where
  T: Terminator<U>,
  S: Selector<U>,
  B: Breeder<U>,
  M: Mutator<U>,
{
  population: Vec<U>,
  archive: Vec<U>,
  archive_size: usize,
  terminator: T,
  selector: S,
  breeder: B,
  mutator: M,
  objectives: Objectives<'a, U>,
}

impl<'a, U, T, S, B, M> Spea2<'a, U, T, S, B, M>
where
  T: Terminator<U>,
  S: Selector<U>,
  B: Breeder<U>,
  M: Mutator<U>,
{
  /// Creates and returns a new `Spea2` struct, performing necessary checks
  /// on creation.
  ///
  /// # Panics
  ///
  /// This function panics if the initial `population` is empty or
  /// `archive_size` is bigger than `population` size or smaller than 0.
  ///
  /// # Examples
  ///
  /// ```
  /// todo!()
  /// ```
  pub fn new(
    population: Vec<U>,
    archive_size: usize,
    terminator: T,
    selector: S,
    breeder: B,
    mutator: M,
    objectives: Objectives<'a, U>,
  ) -> Self {
    assert!(!population.is_empty(), "initial population cannot be empty");
    assert!(
      archive_size > 0 && archive_size <= population.len(),
      "archive size cannot be bigger than initial population size or 0"
    );
    Self {
      population,
      archive_size,
      terminator,
      selector,
      breeder,
      mutator,
      objectives,
      archive: Vec::new(),
    }
  }

  /// Runs the algorithm until termination condition is met. Returns a
  /// non-dominated (might contain dominated sometimes) set of solutions.
  /// Returned solutions are moved out from `Spea2` struct which makes it
  /// unusable, that's why `run` consumes `self`.
  pub fn run(mut self) -> Vec<U> {
    loop {
      if let Some(solutions) = self.run_once() {
        return solutions;
      }
    }
  }

  /// Performs a single algorithm iteration. If termination condition was met on
  /// this iteration, returns a nondominated set of solutions. Otherwise,
  /// returns `None`.
  fn run_once(&mut self) -> Option<Vec<U>> {
    // gather all solutions in a vector
    let mut all_solutions = std::mem::take(&mut self.population);
    all_solutions.append(&mut std::mem::take(&mut self.archive));

    let objective_results = self.objective_values(&all_solutions);
    let raw_fitness = Self::raw_fitness_values(&objective_results);

    let k = (all_solutions.len() as f32).sqrt() as usize;

    // instead of packing solutions and their fitness scores together
    // maybe use consecutive sorting here instead
    let solutions_fitness = all_solutions
      .into_iter()
      .enumerate()
      .map(|(i, s)| (s, raw_fitness[i]))
      .collect::<Vec<_>>();

    // move all nondominated solutions into a new archive:
    // - if there are too many solutions, truncate `nondominated` vector
    // - if there are too few solutions, fill up the archive with dominated ones
    // - otherwise just move nondominated solutions into new archive
    let new_archive = self.do_enviromental_selection(
      solutions_fitness, //
      &objective_results,
      k,
    );

    // check termination condition. if it's met, return best solutions
    if self.terminator.terminate(&new_archive) {
      return Some(new_archive);
    }

    // select, breed and mutate solutions
    let selected_solutions = self.selector.select(&new_archive);
    let mut new_solutions = self.breeder.breed(selected_solutions);
    // TODO: test `par` efficiency
    new_solutions
      .iter_mut()
      .for_each(|s| self.mutator.mutate(s));

    // set new solutions set as current population, update archive
    self.population = new_solutions;
    self.archive = new_archive;

    None
  }

  /// Calculates objective values for given solutions.
  fn objective_values(&self, solutions: &[U]) -> Vec<ObjResults> {
    // TODO: test `par` efficiency
    solutions
      .iter()
      .map(|s| {
        ObjResults(self.objectives.0.iter().map(|o| o.test(s)).collect())
      })
      .collect()
  }

  /// Calculates raw fitness values for each solution.
  /// Raw fitness is determined by the strengths of its dominators in archive
  /// and population.
  fn raw_fitness_values(obj_results: &[ObjResults]) -> Vec<f32> {
    let mut res = vec![0.0; obj_results.len()];
    // TODO: try to parallelize this
    for (i, a) in obj_results[..obj_results.len() - 1].iter().enumerate() {
      for (j, b) in obj_results[i + 1..].iter().enumerate() {
        match a.pareto_dominance_ord(b) {
          Ordering::Less => res[j] += 1.0,
          Ordering::Greater => res[i] += 1.0,
          Ordering::Equal => (),
        };
      }
    }
    res
  }

  /// Calculates distances from `nondom_count` number of vectors to its k-th
  /// neighbor.
  fn distances_to_kth_neighbor(
    obj_results: &[ObjResults],
    nondom_count: usize,
    k: usize,
  ) -> Vec<f32> {
    debug_assert!(k < obj_results.len(), "`k` is out of bounds!");
    debug_assert!(
      nondom_count <= obj_results.len(),
      "`nondom_count` is out of bounds!"
    );

    let mut res = vec![Vec::with_capacity(obj_results.len() - 1); nondom_count];
    // TODO: try to parallelize this
    for (i, a) in obj_results[..nondom_count].iter().enumerate() {
      for (j, b) in obj_results[i + 1..].iter().enumerate() {
        let d = a.distance(b);
        res[i].push(d);
        if j < nondom_count {
          res[j].push(d);
        }
      }
    }

    // TODO: test `par` efficiency
    res
      .iter_mut()
      .map(|v| {
        v.sort_by(|a, b| a.total_cmp(b));
        v[k]
      })
      .collect()
  }

  /// Performs enviromental selecton step:
  /// - if there are too many non-dominated solutions, applies truncation
  ///   operator to them
  /// - if there are too few non-dominated solutions, fill up the archive with
  ///   dominated ones
  /// - otherwise just move nondominated solutions into new archive
  ///
  /// Returns a new archive.
  fn do_enviromental_selection(
    &self,
    mut solutions_fitness: Vec<(U, f32)>, // solution-fitness pairs
    obj_results: &[ObjResults],
    k: usize,
  ) -> Vec<U> {
    // sort solutions by their fitness. nondominated solutions end up
    // in the beginning of the vector
    // TODO: test `par` efficiency
    solutions_fitness.sort_by(|a, b| a.1.total_cmp(&b.1));

    // count first nondominated solutions. counts all such solutions since they
    // are sorted by dominance. a solution is nondominated if its fitness score
    // is < 1
    let nondom_cnt = solutions_fitness.partition_point(|t| t.1 < 1.0);
    if nondom_cnt > self.archive_size {
      // all solutions are non-dominated and their raw strengths are 0
      solutions_fitness
        .iter_mut()
        .zip(Self::distances_to_kth_neighbor(obj_results, nondom_cnt, k))
        .for_each(|((_, f), d)| *f = d);
      // solutions with smaller distance will be in the end of the vector
      // TODO: test `par` efficiency
      solutions_fitness.sort_by(|a, b| b.1.total_cmp(&a.1));
    }

    // if there was too many solutions, takes those with smaller raw strength
    // if there was too few, takes those with bigger distance to k-th neighbor
    solutions_fitness.truncate(self.archive_size);
    debug_assert_eq!(solutions_fitness.len(), self.archive_size);
    // TODO: test `par` efficiency
    solutions_fitness
      .into_iter()
      .map(|t| t.0)
      .collect::<Vec<_>>()
  }
}

/// Containes boxed `Objective`s.
pub struct Objectives<'a, U>(pub Vec<Box<dyn Objective<U> + Send + Sync + 'a>>);

impl<'a, T, U, const N: usize> From<[T; N]> for Objectives<'a, U>
where
  T: Objective<U> + Send + Sync + 'a,
{
  fn from(value: [T; N]) -> Self {
    Objectives(
      value
        .into_iter()
        .map(|v| Box::new(v) as Box<dyn Objective<U> + Send + Sync>)
        .collect::<Vec<_>>(),
    )
  }
}

/// Represents objective results values.
struct ObjResults(Vec<f32>);

impl ObjResults {
  /// Calculates pareto dominance ordering. Returns
  /// - `Less` if `self` dominates `other`
  /// - `Greater` if `other` dominates `self`
  /// - `Equal` otherwise
  fn pareto_dominance_ord(&self, other: &Self) -> Ordering {
    let mut ord = Ordering::Equal;
    // TODO: test `par` efficiency
    for (s, o) in self.0.iter().zip(other.0.iter()) {
      let ord_i = s.abs().total_cmp(&o.abs());
      match (ord_i, ord) {
        (Ordering::Greater, Ordering::Less)
        | (Ordering::Less, Ordering::Greater) => return Ordering::Equal,
        (Ordering::Equal, _) => (),
        (_, Ordering::Equal) => ord = ord_i,
        _ => (),
      }
    }
    ord
  }

  /// Returns `true` if `self` dominates `other`.
  fn pareto_dominates(&self, other: &Self) -> bool {
    matches!(self.pareto_dominance_ord(other), Ordering::Less)
  }

  /// Calculates squared euclidean distance between results' vectors.
  fn distance(&self, other: &Self) -> f32 {
    self
      .0
      .iter()
      .zip(other.0.iter())
      .map(|(a_i, b_i)| (a_i - b_i).powf(2.0))
      .sum()
  }
}
