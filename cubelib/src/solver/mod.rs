use std::vec;
use log::debug;
use crate::cube::Cube333;

use crate::algs::Algorithm;

use crate::solver::solution::Solution;

use crate::steps;

pub mod lookup_table;
pub mod stream;
pub mod solution;
pub mod df_search;
pub mod moveset;
use crate::solver::df_search::CancelToken;
use crate::steps::step::{DefaultStepOptions, Step};

pub fn solve_steps<'a>(puzzle: Cube333, steps: &'a Vec<(Step<'a>, DefaultStepOptions)>, cancel_token: &'a CancelToken) -> impl Iterator<Item = Solution> + 'a {
    let first_step: Box<dyn Iterator<Item = Solution>> = Box::new(vec![Solution::new()].into_iter());

    let solutions: Box<dyn Iterator<Item=Solution>> = steps.iter()
        .fold(first_step, |acc, (step, search_opts)|{
            debug!("Step {} with options {:?}", step.kind(), search_opts);
            let next = steps::step::next_step(acc, step, search_opts.clone(), puzzle.clone(), cancel_token)
                .zip(0..)
                .take_while(|(_, count)| search_opts.step_limit.map(|limit| limit > *count).unwrap_or(true))
                .map(|(sol, _)|sol);
            Box::new(next)
        });

    solutions
}


pub fn solve_steps_vec<'a>(puzzle: Cube333, steps: &'a Vec<(Step<'a>, DefaultStepOptions)>, cancel_token: &'a CancelToken) -> Vec<Solution> {
    let mut solutions = vec![Solution::new()];

    for (step, search_opts) in steps {
        debug!("Step {} with options {:?}", step.kind(), search_opts);
        let mut next_step_solutions = vec![];
        for i in 0..solutions.len() {
            let n = next_step_solutions.len();
            next_step_solutions.extend(
                steps::step::next_step(
                    solutions[i..i+1].iter().map(|s| s.clone()),
                    step,
                    search_opts.clone(),
                    puzzle.clone(),
                    cancel_token
                )
            );
            debug!("Found {} {} for {}", next_step_solutions.len() - n, step.kind(), Into::<Algorithm>::into(solutions[i].clone()));
        }
        let limit = std::cmp::min(search_opts.step_limit.unwrap_or(usize::MAX), next_step_solutions.len());
        debug!("Sampling {}/{} solutions", limit, next_step_solutions.len());
        next_step_solutions.sort_by(|a, b| a.len().cmp(&b.len()));
        solutions = next_step_solutions[..limit].to_vec();
    }

    solutions
}

pub struct SolutionIterator<'a> {
    #[allow(unused)]
    steps: Vec<(Step<'a>, DefaultStepOptions)>,
    solutions: Box<dyn Iterator<Item=Solution>>,
}

impl Iterator for SolutionIterator<'_> {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {
        self.solutions.next()
    }
}
