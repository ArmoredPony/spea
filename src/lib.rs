use std::cmp::Ordering;

use crossover::Crossover;
use mutator::Mutator;
use objective::Objectives;
use rayon::prelude::*;
use selector::Selector;
use terminator::Terminator;

pub mod crossover;
pub mod mutator;
pub mod objective;
pub mod selector;
pub mod terminator;

pub struct Spea2<'a, U, T, S, C, M>
where
  T: Terminator<U>,
  S: Selector<U>,
  C: Crossover<U>,
  M: Mutator<U>,
{
  population: Vec<SolutionData<U>>,
  archive: Vec<SolutionData<U>>,
  archive_size: usize,
  terminator: T,
  selector: S,
  crossover: C,
  mutator: M,
  objectives: Objectives<'a, U>,
  finished: bool,
}

impl<'a, U, T, S, C, M> Spea2<'a, U, T, S, C, M>
where
  T: Terminator<U>,
  S: Selector<U>,
  C: Crossover<U>,
  M: Mutator<U>,
{
  /// Creates and returns a new `Spea2` struct, performing necessary checks
  /// on creation.
  ///
  /// `archive_size` cannot be bigger than initial population size. This is to
  /// ensure that archive is always full.
  ///
  /// # Panics
  ///
  /// This function panics if the initial `population` is empty or
  /// `archive_size` is bigger than `population` size or smaller than 0.
  ///
  /// # Examples
  ///
  /// TODO
  pub fn new(
    population: Vec<U>,
    archive_size: usize,
    terminator: T,
    selector: S,
    crossover: C,
    mutator: M,
    objectives: Objectives<'a, U>,
  ) -> Self {
    assert!(!population.is_empty(), "initial population cannot be empty");
    assert!(
      archive_size > 0 && archive_size <= population.len(),
      "archive size cannot be bigger than initial population size or 0"
    );
    Self {
      archive_size,
      terminator,
      selector,
      crossover,
      mutator,
      objectives,
      population: population.into_iter().map(Into::into).collect(),
      archive: Vec::new(),
      finished: false,
    }
  }

  /// Runs the algorithm until termination condition is met. Returns a
  /// non-dominated (might contain dominated sometimes) set of solutions.
  /// Returned solutions are moved out from `Spea2` struct which makes it
  /// unusable, that's why `run` consumes `self`.
  pub fn run(&mut self) {
    while !self.finished {
      self.run_once()
    }
  }

  /// Performs a single algorithm iteration. If termination condition was met on
  /// this iteration, returns a nondominated set of solutions. Otherwise,
  /// returns `None`.
  pub fn run_once(&mut self) {
    if self.finished {
      return;
    }

    let mut solutions = std::mem::take(&mut self.population);
    solutions.iter_mut().for_each(|s| {
      s.scores = ObjScores(self.objectives.test_each(&s.solution))
    });
    Self::assign_raw_fitness(&mut solutions);
    solutions.append(&mut self.archive);

    self.perform_enrironmental_selection(&mut solutions);
    debug_assert_eq!(solutions.len(), self.archive_size);

    let solution_ptrs: Vec<_> = solutions.iter().map(|s| &s.solution).collect();

    if self.terminator.terminate(&solution_ptrs) {
      self.finished = true;
    }

    let selected_solutions = self.selector.select(&solution_ptrs);

    let mut new_solutions = self.crossover.recombine(&selected_solutions);

    new_solutions
      .iter_mut()
      .for_each(|s| self.mutator.mutate(s));

    self.population = new_solutions.into_iter().map(Into::into).collect();
    self.archive = solutions;
  }

  pub fn is_finished(&self) -> bool {
    self.finished
  }

  /// Moves out all found solutions.
  pub fn get_all_solutions(self) -> Vec<U> {
    self
      .population
      .into_iter()
      .chain(self.archive)
      .map(|s| s.solution)
      .collect()
  }

  /// Moves out all found nondominated solutions **from the archive**.
  pub fn get_nondominated_solutions(self) -> Vec<U> {
    let mut solutions = self.archive;
    solutions.sort_by(|a, b| a.raw_fitness.total_cmp(&b.raw_fitness));
    let nondom_cnt = solutions.partition_point(|s| s.raw_fitness < 1.0);
    solutions.truncate(nondom_cnt);
    solutions.into_iter().map(|s| s.solution).collect()
  }

  /// Returns a slice of all found solutions.
  pub fn peek_all_solutions(&self) -> Vec<&U> {
    self
      .population
      .iter()
      .chain(&self.archive)
      .map(|s| &s.solution)
      .collect()
  }

  /// Returns a slice of all nondominated solutions **from the archive**.
  pub fn peek_nondominated_solutions(&self) -> Vec<&U> {
    self.archive[..self.archive.partition_point(|s| s.raw_fitness < 1.0)]
      .iter()
      .map(|s| &s.solution)
      .collect()
  }

  /// Calculates and assignes raw strength values to solutions' metadata.
  fn assign_raw_fitness(solutions: &mut [SolutionData<U>]) {
    // TODO: parallelize
    let mut strength_vals = vec![0.0; solutions.len()];
    for (i, a) in solutions[..solutions.len() - 1].iter().enumerate() {
      for (j, b) in solutions[i + 1..].iter().enumerate() {
        match a.scores.pareto_dominance_ord(&b.scores) {
          Ordering::Less => strength_vals[i] += 1.0,
          Ordering::Greater => strength_vals[i + j + 1] += 1.0,
          Ordering::Equal => (),
        }
      }
    }
    let mut raw_fitness_vals = vec![0.0; solutions.len()];
    for (i, a) in solutions[..solutions.len() - 1].iter().enumerate() {
      for (j, b) in solutions[i + 1..].iter().enumerate() {
        match a.scores.pareto_dominance_ord(&b.scores) {
          Ordering::Less => raw_fitness_vals[i + j + 1] += strength_vals[i],
          Ordering::Greater => raw_fitness_vals[i] += strength_vals[i + j + 1],
          Ordering::Equal => (),
        }
      }
    }
    for (s, r) in solutions.iter_mut().zip(raw_fitness_vals) {
      s.raw_fitness = r;
    }
  }

  /// Performs enviromental selection on found set of solutions.
  /// Either truncates them if there are too many nondominated ones, or if
  /// there are too few, adds dominated solutions to fill up the archive.
  /// Returns a new archive.
  fn perform_enrironmental_selection(
    &self,
    solutions: &mut Vec<SolutionData<U>>,
  ) {
    // TODO: parallelize
    solutions.sort_unstable_by(|a, b| a.raw_fitness.total_cmp(&b.raw_fitness));
    let nondom_cnt = solutions.partition_point(|s| s.raw_fitness < 1.0);
    if nondom_cnt > self.archive_size {
      self.assign_densities(solutions);
      // TODO: parallelize
      solutions.sort_unstable_by(|a, b| {
        (a.density + a.raw_fitness).total_cmp(&(b.density + b.raw_fitness))
      });
    }
    solutions.truncate(self.archive_size);
  }

  /// Calculates and assignes density values to solutions' metadata.
  fn assign_densities(&self, solutions: &mut [SolutionData<U>]) {
    // TODO: parallelize
    let mut distances = vec![Vec::<f32>::new(); solutions.len()];
    for (i, a) in solutions[..solutions.len() - 1].iter().enumerate() {
      for (j, b) in solutions[i + 1..].iter().enumerate() {
        let d = a.scores.distance(&b.scores);
        distances[i].push(d);
        distances[i + j + 1].push(d);
      }
    }

    let k = (solutions.len() as f32).sqrt() as usize;
    // TODO: parallelize
    for (d, s) in distances.iter_mut().zip(solutions.iter_mut()) {
      d.sort_unstable_by(|a, b| a.total_cmp(b));
      s.density = 1.0 / (d[k] + 2.0);
    }
  }
}

/// Contains a solution and its metadata.
#[derive(Debug)]
struct SolutionData<U> {
  solution: U,
  scores: ObjScores,
  raw_fitness: f32,
  density: f32,
}

impl<U> From<U> for SolutionData<U> {
  fn from(solution: U) -> Self {
    SolutionData {
      solution,
      scores: Default::default(),
      raw_fitness: f32::INFINITY,
      density: f32::INFINITY,
    }
  }
}

/// Represents objective scores values.
#[derive(Debug, Default)]
struct ObjScores(Vec<f32>);

impl ObjScores {
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
        (Ordering::Equal, _) => (),
        (_, Ordering::Equal) => ord = ord_i,
        (Ordering::Greater, Ordering::Less)
        | (Ordering::Less, Ordering::Greater) => return Ordering::Equal,
        _ => (),
      }
    }
    ord
  }

  /// Calculates squared euclidean distance between scores' vectors.
  fn distance(&self, other: &Self) -> f32 {
    self
      .0
      .iter()
      .zip(other.0.iter())
      .map(|(a_i, b_i)| (a_i - b_i).powf(2.0))
      .sum()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pareto_dominance_ord() {
    let a = ObjScores(vec![0.0, 2.0]);
    let b = ObjScores(vec![1.0, 3.0]);
    let c = ObjScores(vec![-1.0, 3.0]);
    let d = ObjScores(vec![2.0, 1.0]);
    let e = ObjScores(vec![-2.0, -1.0]);
    let f = ObjScores(vec![-2.0, -3.0]);
    assert_eq!(a.pareto_dominance_ord(&b), Ordering::Less);
    assert_eq!(b.pareto_dominance_ord(&a), Ordering::Greater);
    assert_eq!(a.pareto_dominance_ord(&c), Ordering::Less);
    assert_eq!(b.pareto_dominance_ord(&d), Ordering::Equal);
    assert_eq!(a.pareto_dominance_ord(&e), Ordering::Equal);
    assert_eq!(a.pareto_dominance_ord(&f), Ordering::Less);
  }

  #[test]
  fn test_distance() {
    let a = ObjScores(vec![0.0, 2.0]);
    let b = ObjScores(vec![1.0, 3.0]);
    let c = ObjScores(vec![-2.0, -1.0]);
    assert_eq!(a.distance(&b), 2.0);
    assert_eq!(b.distance(&a), 2.0);
    assert_eq!(b.distance(&c), 25.0);
    assert_eq!(a.distance(&c), 13.0);
  }
}
