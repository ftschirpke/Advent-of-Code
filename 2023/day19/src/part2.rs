use std::collections::HashMap;

use aoclib::AocError;

use crate::datastructures::{PartCategory, RuleComparator, Workflow, WorkflowAction, WorkflowRule};
use crate::parsing::parse_input;

const RANGE_MIN: u64 = 1;
const RANGE_MAX: u64 = 4000;

#[derive(Debug, Copy, Clone)]
pub struct Range {
    min: u64,
    max: u64,
}

impl Range {
    pub fn new(min: u64, max: u64) -> Self {
        Range {
            min: min.max(RANGE_MIN),
            max: max.min(RANGE_MAX),
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            min: self.min.max(other.min),
            max: self.max.min(other.max),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.min > self.max
    }

    pub fn width(&self) -> u64 {
        if self.is_empty() {
            0
        } else {
            self.max - self.min + 1
        }
    }
}

impl Default for Range {
    fn default() -> Self {
        Self::new(1, 4000)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    pub fn from_condition(category: PartCategory, comp: RuleComparator, val: u32) -> Self {
        let range = match comp {
            RuleComparator::Less => Range::new(RANGE_MIN, val as u64 - 1),
            RuleComparator::Greater => Range::new(val as u64 + 1, RANGE_MAX),
        };
        match category {
            PartCategory::XtremelyGoodLooking => Self {
                x: range,
                m: Range::default(),
                a: Range::default(),
                s: Range::default(),
            },
            PartCategory::Musical => Self {
                x: Range::default(),
                m: range,
                a: Range::default(),
                s: Range::default(),
            },
            PartCategory::Aerodynamic => Self {
                x: Range::default(),
                m: Range::default(),
                a: range,
                s: Range::default(),
            },
            PartCategory::Shiny => Self {
                x: Range::default(),
                m: Range::default(),
                a: Range::default(),
                s: range,
            },
        }
    }

    pub fn from_opposite_condition(category: PartCategory, comp: RuleComparator, val: u32) -> Self {
        match comp {
            RuleComparator::Less => {
                Self::from_condition(category, RuleComparator::Greater, val - 1)
            }
            RuleComparator::Greater => {
                Self::from_condition(category, RuleComparator::Less, val + 1)
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.x.is_empty() || self.m.is_empty() || self.a.is_empty() || self.s.is_empty()
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            x: self.x.intersection(&other.x),
            m: self.m.intersection(&other.m),
            a: self.a.intersection(&other.a),
            s: self.s.intersection(&other.s),
        }
    }

    pub fn count_combinations(&self) -> u64 {
        self.x.width() * self.m.width() * self.a.width() * self.s.width()
    }
}

pub fn calculate_workflow_action_ranges(
    workflows: &[Workflow],
) -> Vec<Vec<(WorkflowAction, PartRange)>> {
    workflows
        .iter()
        .map(|wf| {
            let mut part_range = PartRange::default(); // range of parts that get to the current rule
            wf.rules
                .iter()
                .filter_map(|rule| {
                    let (range_for_action, action) = match *rule {
                        WorkflowRule::Conditional {
                            category,
                            comp,
                            val,
                            ref action,
                        } => {
                            let condition_true_range =
                                PartRange::from_condition(category, comp, val)
                                    .intersection(&part_range);
                            part_range = PartRange::from_opposite_condition(category, comp, val)
                                .intersection(&part_range);
                            (condition_true_range, action.clone())
                        }
                        WorkflowRule::Otherwise(ref action) => (part_range, action.clone()),
                    };
                    if action == WorkflowAction::Reject {
                        None
                    } else {
                        Some((action, range_for_action))
                    }
                })
                .collect()
        })
        .collect()
}

pub fn process(input: &'static str) -> Result<u64, AocError> {
    let (workflows, _) = parse_input(input)?;

    // which ranges cause which actions for each workflow
    let action_ranges_for_workflows = calculate_workflow_action_ranges(&workflows);

    // instead of single parts, we now process ranges of parts through the workflows and start with
    // one range that covers all parts at the "in" workflow
    let mut ranges_at_workflow: Vec<Vec<PartRange>> = vec![Vec::new(); workflows.len()];
    let workflow_indices: HashMap<_, _> = workflows
        .iter()
        .enumerate()
        .map(|(i, workflow)| (workflow.name.clone(), i))
        .collect();
    let in_index = *workflow_indices
        .get("in")
        .ok_or_else(|| AocError::LogicError("No 'in' workflow found".to_string()))?;
    ranges_at_workflow[in_index].push(PartRange::default());

    let mut accepted_ranges: Vec<PartRange> = Vec::new();
    let mut iterations = 0;

    // while there are still ranges to process
    while ranges_at_workflow.iter().any(|ranges| !ranges.is_empty()) {
        if iterations > 10000000 {
            return Err(AocError::LogicError(
                "Too many iterations, probably infinite loop".to_string(),
            ));
        }
        iterations += 1;
        // collect the locations where all ranges end up after this processing step
        let mut new_ranges_at_workflow: Vec<Vec<PartRange>> = vec![Vec::new(); workflows.len()];
        ranges_at_workflow
            .iter_mut()
            .zip(&action_ranges_for_workflows)
            .for_each(|(ranges, workflow_action_ranges)| {
                // combine all ranges at this current workflow with all the ranges that cause the
                // different possible actions
                ranges.drain(..).for_each(|range| {
                    workflow_action_ranges
                        .iter()
                        .for_each(|(action, range_for_action)| {
                            let new_range = range.intersection(range_for_action);
                            // if the resulting intersection is not empty, perform the action
                            if !new_range.is_empty() {
                                match *action {
                                    WorkflowAction::Accept => accepted_ranges.push(new_range),
                                    WorkflowAction::JumpTo(ref next_workflow) => {
                                        let next_workflow_idx =
                                            *workflow_indices.get(next_workflow).unwrap();
                                        new_ranges_at_workflow[next_workflow_idx].push(new_range);
                                    }
                                    WorkflowAction::Reject => unreachable!(), // already filtered out
                                }
                            }
                        });
                })
            });
        ranges_at_workflow = new_ranges_at_workflow;
    }

    // the way we processed the ranges, there might be overlapping ranges
    // thus, we need to count the combinations of each range and subtract the overlap
    let mut processed_ranges: Vec<PartRange> = Vec::new();
    let distinct_combinations = accepted_ranges
        .drain(..)
        .map(|accepted_range| {
            let range_combinations = accepted_range.count_combinations();
            let already_counted_combinations: u64 = processed_ranges
                .iter()
                .map(|already_processed_range| {
                    accepted_range
                        .intersection(already_processed_range)
                        .count_combinations()
                })
                .sum();
            processed_ranges.push(accepted_range);
            range_combinations - already_counted_combinations
        })
        .sum();
    Ok(distinct_combinations)
}
