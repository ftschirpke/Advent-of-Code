use std::collections::HashMap;

use aoclib::AocError;

use crate::datastructures::{
    Part, PartCategory, RuleComparator, Workflow, WorkflowAction, WorkflowRule,
};
use crate::parsing::parse_input;

fn compare(first_val: u32, comp: RuleComparator, second_val: u32) -> bool {
    match comp {
        RuleComparator::Less => first_val < second_val,
        RuleComparator::Greater => first_val > second_val,
    }
}

impl Workflow {
    pub fn action(&self, part: &Part) -> Option<WorkflowAction> {
        self.rules.iter().find_map(|rule| rule.action(part))
    }
}

impl WorkflowRule {
    pub fn action(&self, part: &Part) -> Option<WorkflowAction> {
        match self {
            Self::Conditional {
                category,
                comp,
                val,
                action,
            } => {
                let fulfills_conditional = match category {
                    PartCategory::XtremelyGoodLooking => compare(part.x, *comp, *val),
                    PartCategory::Musical => compare(part.m, *comp, *val),
                    PartCategory::Aerodynamic => compare(part.a, *comp, *val),
                    PartCategory::Shiny => compare(part.s, *comp, *val),
                };
                if fulfills_conditional {
                    Some(action.clone())
                } else {
                    None
                }
            }
            Self::Otherwise(action) => Some(action.clone()),
        }
    }
}

impl Part {
    pub fn sum_ratings(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

pub fn do_workflows_accept_part(workflows: &[Workflow], part: &Part) -> bool {
    let workflow_names: HashMap<_, _> = workflows
        .iter()
        .enumerate()
        .map(|(i, workflow)| (workflow.name.clone(), i))
        .collect();
    let mut workflow_name: String = String::from("in");
    loop {
        let workflow_idx = workflow_names.get(&workflow_name);
        if let Some(&idx) = workflow_idx {
            let workflow = &workflows[idx];
            if let Some(action) = workflow.action(part) {
                match action {
                    WorkflowAction::Accept => break true,
                    WorkflowAction::Reject => break false,
                    WorkflowAction::JumpTo(next_workflow) => workflow_name = next_workflow.clone(),
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

pub fn process(input: &'static str) -> Result<u32, AocError> {
    let (workflows, parts) = parse_input(input)?;
    let accepted_parts = parts
        .iter()
        .filter(|part| do_workflows_accept_part(&workflows, part));
    let result = accepted_parts.map(|part| part.sum_ratings()).sum();
    Ok(result)
}
