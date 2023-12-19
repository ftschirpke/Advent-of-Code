#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowAction {
    Accept,
    Reject,
    JumpTo(String),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RuleComparator {
    Less,
    Greater,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PartCategory {
    XtremelyGoodLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowRule {
    Conditional {
        category: PartCategory,
        comp: RuleComparator,
        val: u32,
        action: WorkflowAction,
    },
    Otherwise(WorkflowAction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workflow {
    pub name: String,
    pub rules: Vec<WorkflowRule>,
}
