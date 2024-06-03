use breeder::Breeder;
use itertools::Itertools;
use mutator::Mutator;
use objective::{Objective, Objectives, ParetoDominance};
use rayon::prelude::*;
use selector::Selector;
use terminator::Terminator;

pub mod breeder;
pub mod mutator;
pub mod objective;
pub mod selector;
pub mod terminator;

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
    let all_solutions: Vec<S> = std::mem::take(&mut self.population)
      .into_iter()
      .chain(std::mem::take(&mut self.archive))
      .collect();

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

    // count nondominated solutions.
    // a solution is nondominated if its fitness score < 1
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
        .filter(|f_j| f_i.as_slice().pareto_dominates(&f_j.as_slice()))
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
