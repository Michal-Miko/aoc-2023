use std::{collections::HashMap, iter::once, path::PathBuf};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use itertools::{repeat_n, Itertools};
use winnow::{
    ascii::{digit1, space1},
    combinator::{empty, fail, repeat, separated, seq},
    dispatch,
    token::any,
    PResult, Parser,
};

pub struct Day12;

#[derive(Clone, Debug, PartialEq, Eq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
struct DPState {
    spring_idx: usize,
    group_idx: usize,
    group_size: usize,
}

impl DPState {
    fn ok(mut self) -> Self {
        self.spring_idx += 1;
        self.group_size = 0;
        self
    }

    fn damaged_group_end(mut self) -> Self {
        self.spring_idx += 1;
        self.group_idx += 1;
        self.group_size = 0;
        self
    }

    fn damaged(mut self) -> Self {
        self.spring_idx += 1;
        self.group_size += 1;
        self
    }
}

#[derive(Clone, Debug)]
enum DFAState {
    LoopOp,
    LoopOpReqDam,
    ReqOp,
    ReqDam,
}

#[derive(Clone, Debug)]
struct SpringRecord {
    springs: Vec<SpringState>,
    damaged_groups: Vec<usize>,
}

impl SpringRecord {
    #[allow(dead_code)]
    fn compute_arrangements_dp(&self) -> usize {
        let mut to_check = vec![DPState::default()];
        let mut parent_states = HashMap::new();
        let mut computed_states: HashMap<DPState, usize> = HashMap::new();

        while let Some(mut current_state) = to_check.pop() {
            let mut valid = 0;
            let mut computed = false;

            let DPState {
                spring_idx,
                group_idx,
                group_size,
            } = current_state;

            // Check if we computed this state already
            if let Some(result) = computed_states.get(&current_state) {
                valid = *result;
                computed = true;
            // Check if current_state is a leaf
            } else if spring_idx == self.springs.len() {
                let no_extra_damaged_springs =
                    group_idx == self.damaged_groups.len() && group_size == 0;
                let last_group_valid = group_idx == self.damaged_groups.len() - 1
                    && group_size == self.damaged_groups[group_idx];
                valid = (no_extra_damaged_springs || last_group_valid) as usize;
                computed = true;
            }
            // Back propagate the results and continue if we've got them already
            if computed {
                while let Some(parent) = parent_states.get(&current_state) {
                    computed_states
                        .entry(*parent)
                        .and_modify(|entry| *entry += valid)
                        .or_insert(valid);
                    current_state = *parent;
                }
                continue;
            }
            // Cache miss and we're not in a leaf - check current spring
            let current_spring = &self.springs[spring_idx];
            for possible_spring_state in [SpringState::Damaged, SpringState::Operational] {
                if possible_spring_state == *current_spring
                    || SpringState::Unknown == *current_spring
                {
                    match (possible_spring_state, group_size) {
                        // No changes
                        (SpringState::Operational, 0) => {
                            let new_state = current_state.ok();
                            to_check.push(new_state);
                            parent_states.insert(new_state, current_state);
                        }
                        // Valid end of damaged group
                        (SpringState::Operational, _)
                            if group_idx < self.damaged_groups.len()
                                && self.damaged_groups[group_idx] == group_size =>
                        {
                            let new_state = current_state.damaged_group_end();
                            to_check.push(new_state);
                            parent_states.insert(new_state, current_state);
                        }
                        // Continuation/Start of damaged group
                        (SpringState::Damaged, _) => {
                            let new_state = current_state.damaged();
                            to_check.push(new_state);
                            parent_states.insert(new_state, current_state);
                        }
                        // Invalid arrangements
                        _ => (),
                    }
                }
            }
        }

        *computed_states
            .get(&DPState::default())
            .expect("The root state should be computed")
    }

    fn compute_arrangements_dfa(&self) -> usize {
        // Defines possible states
        let states = Itertools::intersperse(
            self.damaged_groups.iter().map(|length| {
                let mut v = vec![DFAState::ReqDam; *length];
                v[0] = DFAState::LoopOpReqDam;
                v
            }),
            vec![DFAState::ReqOp],
        )
        .chain(once(vec![DFAState::LoopOp]))
        .flatten()
        .collect::<Vec<_>>();
        // Tracks how many possible arrangements are currently in each state
        let mut current = vec![0usize; states.len()];
        let mut next = vec![0usize; states.len()];
        current[0] = 1; // Input arrangement in the first state

        for spring in self.springs.iter() {
            for (i, state) in states.iter().enumerate() {
                match (current[i], spring, state) {
                    // Skip states without any arrangements
                    (0, _, _) => (),
                    // Operational springs loop before blocks / in end state
                    (
                        arrangements,
                        SpringState::Operational,
                        DFAState::LoopOpReqDam | DFAState::LoopOp,
                    ) => next[i] += arrangements,
                    // Advance - Operational springs end a Damaged block
                    (arrangements, SpringState::Operational, DFAState::ReqOp) => {
                        next[i + 1] += arrangements
                    }
                    // Advance - Damaged springs extend a block or start a new block
                    (
                        arrangements,
                        SpringState::Damaged,
                        DFAState::LoopOpReqDam | DFAState::ReqDam,
                    ) => next[i + 1] += arrangements,
                    // Operational loop before a block, Damaged advance starting a block
                    (arrangements, SpringState::Unknown, DFAState::LoopOpReqDam) => {
                        next[i] += arrangements;
                        next[i + 1] += arrangements;
                    }
                    // Operational loop, Damaged terminate - end state
                    (arrangements, SpringState::Unknown, DFAState::LoopOp) => {
                        next[i] += arrangements;
                    }
                    // Operational terminate, Damaged advance - in the middle of a block
                    // or
                    // Operational advance, Damaged terminate - finishing a block
                    (arrangements, SpringState::Unknown, DFAState::ReqDam | DFAState::ReqOp) => {
                        next[i + 1] += arrangements;
                    }
                    // Terminate all of the remaining invalid arrangements
                    (_, SpringState::Operational | SpringState::Damaged, _) => (),
                }
            }
            std::mem::swap(&mut current, &mut next);
            next.fill(0);
        }
        *current.last().expect("SpringRecord shouldn't be empty")
    }

    fn multiply_by(mut self, mult: usize) -> Self {
        self.springs =
            Itertools::intersperse(repeat_n(self.springs, mult), vec![SpringState::Unknown])
                .flatten()
                .collect();
        self.damaged_groups = repeat_n(self.damaged_groups, mult).flatten().collect();
        self
    }
}

fn parse_spring(input: &mut &str) -> PResult<SpringState> {
    dispatch! {any;
        '.' => empty.value(SpringState::Operational),
        '#' => empty.value(SpringState::Damaged),
        '?' => empty.value(SpringState::Unknown),
        _ => fail,
    }
    .parse_next(input)
}

fn parse_spring_row(input: &mut &str) -> PResult<Vec<SpringState>> {
    repeat(1.., parse_spring).parse_next(input)
}

fn parse_spring_groups(input: &mut &str) -> PResult<Vec<usize>> {
    separated(1.., digit1.parse_to::<usize>(), ',').parse_next(input)
}

fn parse_spring_record(input: &mut &str) -> PResult<SpringRecord> {
    seq! {SpringRecord{
        springs: parse_spring_row,
        _: space1,
        damaged_groups: parse_spring_groups
    }}
    .parse_next(input)
}

impl AocTask for Day12 {
    fn directory(&self) -> PathBuf {
        "tasks/day_12".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let records = input.map(|input| {
            parse_spring_record
                .parse(&input)
                .map_err(|err| err.to_string())
        });

        match phase {
            1 => records
                .map_ok(|record| record.compute_arrangements_dfa())
                .process_results(|iter| iter.sum::<usize>()),
            _ => records
                .map_ok(|record| record.multiply_by(5).compute_arrangements_dfa())
                .process_results(|iter| iter.sum::<usize>()),
        }
        .try_solved()
    }
}
