use std::cmp::min;
use std::env::var;
use std::marker::PhantomData;

use crate::algs::{Algorithm, Solution};
use crate::coords::coord::Coord;
use crate::cube::{ApplyAlgorithm, Invertible, Transformation, Turnable};
use crate::cube::Turn::Half;
use crate::df_search::{dfs_iter, NissType};
use crate::lookup_table::PruningTable;
use crate::moveset::MoveSet;
use crate::stream;


#[derive(Clone, Copy, Debug)]
pub struct DefaultStepOptions {
    pub niss_type: NissType,
    pub min_moves: u8,
    pub max_moves: u8,
    pub step_quality: Option<usize>,
    pub step_limit: Option<usize>,
}

pub trait StepVariant<CubeParam>:
    PreStepCheck<CubeParam> +
    PostStepCheck<CubeParam>
{
    fn move_set(&self, cube: &CubeParam, depth_left: u8) -> &'_ MoveSet;
    fn pre_step_trans(&self) -> &'_ Vec<Transformation>;
    fn heuristic(&self, cube: &CubeParam, depth_left: u8, can_niss: bool) -> u8;
    fn name(&self) -> &str;
    fn is_half_turn_invariant(&self) -> bool;
}

pub trait PreStepCheck<CubeParam> {
    fn is_cube_ready(&self, cube: &CubeParam) -> bool;
}

pub trait PostStepCheck<CubeParam> {
    fn is_solution_admissible(&self, cube: &CubeParam, alg: &Algorithm) -> bool;
}

#[derive(Copy, Clone)]
pub struct AnyPostStepCheck;

impl <CubeParam> PostStepCheck<CubeParam> for AnyPostStepCheck {
    fn is_solution_admissible(&self, cube: &CubeParam, alg: &Algorithm) -> bool {
        true
    }
}

pub(crate) struct DefaultPruningTableStep<'a, const HC_SIZE: usize, HC: Coord<HC_SIZE>, const PC_SIZE: usize, PC: Coord<PC_SIZE>, CubeParam, PSC: PostStepCheck<CubeParam>>
    where
        HC: for<'x> From<&'x CubeParam>,
        PC: for<'x> From<&'x CubeParam>,
{
    move_set: &'a MoveSet,
    pre_trans: Vec<Transformation>,
    table: &'a PruningTable<HC_SIZE, HC>,
    name: &'a str,
    post_step_checker: PSC,
    _pc: PhantomData<PC>,
    _cube: PhantomData<CubeParam>,
}

impl <'a, const HC_SIZE: usize, HC: Coord<HC_SIZE>, const PC_SIZE: usize, PC: Coord<PC_SIZE>, CubeParam, PSC: PostStepCheck<CubeParam>> PreStepCheck<CubeParam> for DefaultPruningTableStep<'a, HC_SIZE, HC, PC_SIZE, PC, CubeParam, PSC>
    where
        HC: for<'x> From<&'x CubeParam>,
        PC: for<'x> From<&'x CubeParam>, {

    fn is_cube_ready(&self, cube: &CubeParam) -> bool {
        PC::from(cube).val() == 0
    }
}

impl <'a, const HC_SIZE: usize, HC: Coord<HC_SIZE>, const PC_SIZE: usize, PC: Coord<PC_SIZE>, CubeParam, PSC: PostStepCheck<CubeParam>> PostStepCheck<CubeParam> for DefaultPruningTableStep<'a, HC_SIZE, HC, PC_SIZE, PC, CubeParam, PSC>
    where
        HC: for<'x> From<&'x CubeParam>,
        PC: for<'x> From<&'x CubeParam>, {

    fn is_solution_admissible(&self, cube: &CubeParam, alg: &Algorithm) -> bool {
        self.post_step_checker.is_solution_admissible(cube, alg)
    }
}

impl <'a, const HC_SIZE: usize, HC: Coord<HC_SIZE>, const PC_SIZE: usize, PC: Coord<PC_SIZE>, CubeParam, PSC: PostStepCheck<CubeParam>> StepVariant<CubeParam> for DefaultPruningTableStep<'a, HC_SIZE, HC, PC_SIZE, PC, CubeParam, PSC>
    where
        HC: for<'x> From<&'x CubeParam>,
        PC: for<'x> From<&'x CubeParam>, {

    fn move_set(&self, cube: &CubeParam, depth_left: u8) -> &'_ MoveSet {
        self.move_set
    }

    fn pre_step_trans(&self) -> &'_ Vec<Transformation> {
        &self.pre_trans
    }

    fn heuristic(&self, cube: &CubeParam, depth_left: u8, can_niss: bool) -> u8 {
        let coord = HC::from(cube);
        let val = self.table.get(coord).expect("Expected table to be filled");
        if can_niss && val != 0 {
            1
        } else {
            val
        }
    }

    fn name(&self) -> &str {
        self.name
    }

    fn is_half_turn_invariant(&self) -> bool {
        !self.move_set.st_moves
            .iter()
            .any(|m| m.1 == Half)
    }
}

impl <'a, const HC_SIZE: usize, HC: Coord<HC_SIZE>, const PC_SIZE: usize, PC: Coord<PC_SIZE>, PSC: PostStepCheck<CubeParam>, CubeParam> DefaultPruningTableStep<'a, HC_SIZE, HC, PC_SIZE, PC, CubeParam, PSC>
    where
        HC: for<'x> From<&'x CubeParam>,
        PC: for<'x> From<&'x CubeParam>, {

    pub fn new(move_set: &'a MoveSet,
               pre_trans: Vec<Transformation>,
               table: &'a PruningTable<HC_SIZE, HC>,
               post_step_checker: PSC,
               name: &'a str) -> Self {
        DefaultPruningTableStep {
            move_set,
            pre_trans,
            table,
            name,
            post_step_checker,
            _cube: PhantomData::default(),
            _pc: PhantomData::default()
        }
    }
}


pub struct Step<'a, CubeParam> {
    step_variants: Vec<Box<dyn StepVariant<CubeParam> + 'a>>,
    name: &'static str,
}

impl<'a, CubeParam: 'a>
    Step<'a, CubeParam>
{
    pub fn new(
        step_variants: Vec<Box<dyn StepVariant<CubeParam> + 'a>>,
        name: &'static str,
    ) -> Self {
        Step { step_variants, name }
    }

    pub fn new_basic<const C_SIZE: usize, CoordParam: Coord<C_SIZE> + 'a>(
        name: &'static str,
        moves: MoveSet,
    ) -> Self
    where
        CoordParam: for<'x> From<&'x CubeParam>,
    {
        let variant: NoHeuristicStep<C_SIZE, CoordParam, CubeParam> =
            NoHeuristicStep {
                move_set: moves,
                trans: vec![],
                name,
                _c: PhantomData::default(),
                _cube: PhantomData::default(),
            };
        Step {
            step_variants: vec![Box::new(variant)],
            name: "unknown",
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn is_half_turn_invariant(&self) -> bool {
        self.step_variants.iter()
            .any(|sv|sv.is_half_turn_invariant())
    }
}

struct NoHeuristicStep<
    const C_SIZE: usize,
    CoordParam: Coord<C_SIZE>,
    CubeParam,
> where
    CoordParam: for<'x> From<&'x CubeParam>,
{
    move_set: MoveSet,
    trans: Vec<Transformation>,
    name: &'static str,
    _c: PhantomData<CoordParam>,
    _cube: PhantomData<CubeParam>,
}

impl<
        const C_SIZE: usize,
        CoordParam: Coord<C_SIZE>,
        CubeParam,
    > PreStepCheck<CubeParam>
    for NoHeuristicStep<C_SIZE, CoordParam, CubeParam>
where
    CoordParam: for<'x> From<&'x CubeParam>,
{
    fn is_cube_ready(&self, _cube: &CubeParam) -> bool {
        true
    }
}

impl<
    const C_SIZE: usize,
    CoordParam: Coord<C_SIZE>,
    CubeParam,
> PostStepCheck<CubeParam>
for NoHeuristicStep<C_SIZE, CoordParam, CubeParam>
    where
        CoordParam: for<'x> From<&'x CubeParam>,
{
    fn is_solution_admissible(&self, cube: &CubeParam, alg: &Algorithm) -> bool {
        true
    }
}

impl<
        const C_SIZE: usize,
        CoordParam: Coord<C_SIZE>,
        CubeParam,
    > StepVariant<CubeParam>
    for NoHeuristicStep<C_SIZE, CoordParam, CubeParam>
where
    CoordParam: for<'x> From<&'x CubeParam>,
{
    fn move_set(&self, cube: &CubeParam, depth_left: u8) -> &'_ MoveSet {
        &self.move_set
    }

    fn pre_step_trans(&self) -> &'_ Vec<Transformation> {
        &self.trans
    }

    fn heuristic(&self, cube: &CubeParam, depth_left: u8, can_niss: bool) -> u8 {
        min(CoordParam::from(cube).val(), 1) as u8
    }

    fn name(&self) -> &str {
        self.name
    }

    fn is_half_turn_invariant(&self) -> bool {
        !self.move_set.st_moves
            .iter()
            .any(|m| m.1 == Half)
    }
}

pub fn first_step<
    'a,
    'b,
    CubeParam: Turnable + Invertible + Copy,
>(
    step: &'a Step<'b, CubeParam>,
    search_opts: DefaultStepOptions,
    cube: CubeParam,
) -> impl Iterator<Item = Solution> + 'a {
    next_step(vec![Solution::new()].into_iter(), step, search_opts, cube)
}

//TODO once we have a better way to merge alg iterators, we should invoke df_search with the full bounds immediately.
//It's not significantly more efficient yet, but in the future it probably will be
pub fn next_step<
    'a,
    'b,
    IN: Iterator<Item = Solution> + 'a,
    CubeParam: Turnable + Invertible + Copy,
>(
    algs: IN,
    step: &'a Step<'b, CubeParam>,
    search_opts: DefaultStepOptions,
    cube: CubeParam,
) -> impl Iterator<Item = Solution> + 'a {
    stream::iterated_dfs(algs, move |solution, depth| {
        let result: Box<dyn Iterator<Item = Solution>> =
            if depth < search_opts.min_moves || depth > search_opts.max_moves {
                Box::new(vec![].into_iter())
            } else {
                let mut cube = cube.clone();
                let alg: Algorithm = solution.clone().into();
                cube.apply_alg(&alg);
                let stage_opts = DefaultStepOptions::new(depth, depth, search_opts.niss_type, search_opts.step_limit, search_opts.step_quality);
                let previous_normal = alg.normal_moves.last().cloned();
                let previous_inverse = alg.inverse_moves.last().cloned();

                //Only allow the first variant to use the empty solution, otherwise we get lots of duplicates
                let values = step
                    .step_variants
                    .iter()
                    .flat_map(move |step_variant| {
                        dfs_iter(
                            step_variant.as_ref(),
                            cube,
                            stage_opts.clone(),
                            previous_normal,
                            previous_inverse,
                        )
                        .map(|alg| (step_variant.name(), alg))
                    })
                    .flat_map(|(name, iter)| iter.map(move |alg| (name, alg)));
                let values: Box<dyn Iterator<Item = (&str, Algorithm)>> = if depth == 0 {
                    Box::new(values.take(100))
                } else if let Some(step_quality) = search_opts.step_quality {
                    Box::new(values.take(step_quality))
                } else {
                    Box::new(values)
                };
                Box::new(values.map(move |(step_name, step_alg)| {
                        let mut sol = solution.clone();
                        sol.add_step(step_name.to_string(), step_alg);
                        sol
                    }))
            };
        result
    })
}
