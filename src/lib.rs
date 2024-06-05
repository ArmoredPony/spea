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

struct Objectives<S>(pub Vec<Box<dyn Objective<S> + Send + Sync>>);

trait ParetoDominance {
  /// Calculates pareto dominance ordering. Returns
  /// - `Less` if `self` dominates `other`
  /// - `Greater` if `other` dominates `self`
  /// - `Equal` otherwise
  fn pareto_dominance_ord(&self, other: &Self) -> Ordering;

  /// Returns `true` if `self` dominates `other`.
  fn pareto_dominates(&self, other: &Self) -> bool {
    matches!(self.pareto_dominance_ord(other), Ordering::Less)
  }
}

impl<T> ParetoDominance for T
where
  T: AsRef<[f32]>,
{
  fn pareto_dominance_ord(&self, other: &Self) -> Ordering {
    let mut ord = Ordering::Equal;
    for (s, o) in self.as_ref().iter().zip(other.as_ref().iter()) {
      let ord_i = s
        .abs()
        .partial_cmp(&o.abs())
        .expect("attempted to compare a NaN");
      match (ord_i, ord) {
        (Ordering::Greater, Ordering::Less)
        | (Ordering::Less, Ordering::Greater) => return Ordering::Equal,
        (_, Ordering::Equal) => ord = ord_i,
        _ => (),
      }
    }
    ord
  }
}

pub struct Spea2<S, T, L, B, M>
where
  T: Terminator<S>,
  L: Selector<S>,
  B: Breeder<S>,
  M: Mutator<S>,
{
  population: Vec<S>,
  archive: Vec<S>,
  archive_size: usize,
  terminator: T,
  selector: L,
  breeder: B,
  mutator: M,
  objectives: Objectives<S>,
}

impl<S, T, L, B, M> Spea2<S, T, L, B, M>
where
  S: Clone + Send + Sync,
  T: Terminator<S> + Sync,
  L: Selector<S>,
  B: Breeder<S>,
  M: Mutator<S> + Send + Sync,
{
  /// Performs a single algorithm iteration. If termination condition was met on
  /// this iteration, returns a nondominated set of solutions. Otherwise,
  /// returns `None`.
  pub fn run_once(&mut self) -> Option<Vec<S>> {
    // gather all solutions in a vector
    let mut all_solutions = std::mem::take(&mut self.population);
    all_solutions.append(&mut std::mem::take(&mut self.archive));

    // calculate fitness values per solution
    let raw_fitness = self.raw_fitness_values(&all_solutions);
    let densities = Self::density_values(&all_solutions);
    let solutions_fitness = all_solutions
      .into_par_iter()
      .enumerate()
      .map(|(i, s)| (s, raw_fitness[i] as f32 + densities[i]))
      .collect::<Vec<_>>();

    // sort solutions by their fitness. nondominated solutions end up
    // in the beginning of the vector
    let mut sorted_solutions_fitness = solutions_fitness;
    sorted_solutions_fitness.par_sort_by(|a, b| {
      a.1.partial_cmp(&b.1).expect("attempted to compare a NaN")
    });

    // count first nondominated solutions. counts all such solution since they
    // are sorted by dominance. a solution is nondominated if its fitness score
    // is < 1
    let nondom_cnt = sorted_solutions_fitness
      .iter()
      .position(|t| t.1 >= 1.0)
      .unwrap_or(sorted_solutions_fitness.len());

    // remove fitness values from solutions vector
    let mut sorted_solutions = sorted_solutions_fitness
      .into_iter()
      .map(|t| t.0)
      .collect::<Vec<_>>();

    // move all nondominated solutions into a new archive:
    // - if there are too many solutions, truncate `nondominated` vector
    // - if there are too few solutions, fill up the archive with dominated ones
    // - otherwise just move nondominated solutions into new archive
    let new_archive: Vec<S> = if nondom_cnt > self.archive_size {
      sorted_solutions.truncate(nondom_cnt);
      self.truncate_closest(sorted_solutions)
    } else {
      sorted_solutions.truncate(self.archive_size);
      sorted_solutions
    };

    // check termination condition. if it's met, return best solutions
    if self.terminator.terminate(&new_archive) {
      return Some(new_archive);
    }

    // select, breed and mutate solutions
    let selected_solutions = self.selector.select(&new_archive);
    let mut new_solutions = self.breeder.breed(selected_solutions);
    new_solutions
      .par_iter_mut()
      .for_each(|s| self.mutator.mutate(s));

    // set new solutions set as current population
    self.population = new_solutions;

    None
  }

  pub fn run(&mut self) -> Vec<S> {
    loop {
      if let Some(solutions) = self.run_once() {
        return solutions;
      }
    }
  }

  /// Truncates `solutions` vector to archive size by removing closest
  /// neighbors.
  fn truncate_closest(&self, solutions: Vec<S>) -> Vec<S> {
    todo!()
  }

  /// Calculates raw fitness values for each solution.
  /// Raw fitness is determined by the strengths of its dominators in archive
  /// and population.
  fn raw_fitness_values(&self, solutions: &[S]) -> Vec<usize> {
    // let strength_values: Vec<usize> = Vec::with_capacity(solutions.len());
    let solutions_obj_fitness = solutions
      .iter()
      .map(|s| self.objectives.0.par_iter().map(|o| o.test(s)).collect())
      .collect::<Vec<Vec<f32>>>();
    // TODO: reimplement with `pareto_dominance_ord`, should be more efficient
    let strength_values = solutions_obj_fitness.iter().map(|f_i| {
      solutions_obj_fitness
        .iter()
        .filter(|f_j| f_i.pareto_dominates(f_j))
        .count()
    });
    todo!()
  }

  /// Calculates density values of solutions.
  /// Each density value is greater than 0 and less than 1.
  fn density_values(solutions: &[S]) -> Vec<f32> {
    todo!()
  }
}
