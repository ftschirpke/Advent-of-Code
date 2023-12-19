use aoclib::AocError;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char as char_parser, line_ending, u32 as u32_parser},
    combinator::{map_res, value},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

use crate::datastructures::{
    Part, PartCategory, RuleComparator, Workflow, WorkflowAction, WorkflowRule,
};

fn is_ascii_lowercase(c: char) -> bool {
    c.is_ascii_lowercase()
}

fn to_string(input: &str) -> Result<String, AocError> {
    Ok(String::from(input))
}

fn parse_workflow_name(input: &str) -> IResult<&str, String> {
    map_res(take_while1(is_ascii_lowercase), to_string)(input)
}

fn parse_rule(input: &str) -> IResult<&str, WorkflowRule> {
    alt((
        parse_conditional_rule,
        map_res(parse_action, |action| {
            Ok::<WorkflowRule, AocError>(WorkflowRule::Otherwise(action))
        }),
    ))(input)
}

fn parse_conditional_rule(input: &str) -> IResult<&str, WorkflowRule> {
    let (input, category) = alt((
        value(PartCategory::XtremelyGoodLooking, char_parser('x')),
        value(PartCategory::Musical, char_parser('m')),
        value(PartCategory::Aerodynamic, char_parser('a')),
        value(PartCategory::Shiny, char_parser('s')),
    ))(input)?;
    let (input, comp) = alt((
        value(RuleComparator::Less, char_parser('<')),
        value(RuleComparator::Greater, char_parser('>')),
    ))(input)?;
    let (input, val) = u32_parser(input)?;
    let (input, action) = preceded(char_parser(':'), parse_action)(input)?;
    let rule = WorkflowRule::Conditional {
        category,
        comp,
        val,
        action,
    };
    Ok((input, rule))
}

fn parse_action(input: &str) -> IResult<&str, WorkflowAction> {
    alt((
        map_res(parse_workflow_name, |name| {
            Ok::<WorkflowAction, AocError>(WorkflowAction::JumpTo(name))
        }),
        value(WorkflowAction::Accept, char_parser('A')),
        value(WorkflowAction::Reject, char_parser('R')),
    ))(input)
}

fn parse_workflow_line(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = parse_workflow_name(input)?;
    let (input, rules) = delimited(
        char_parser('{'),
        separated_list1(char_parser(','), parse_rule),
        char_parser('}'),
    )(input)?;
    let (input, _) = line_ending(input)?;
    let workflow = Workflow { name, rules };
    Ok((input, workflow))
}

fn parse_part_line(input: &str) -> IResult<&str, Part> {
    let (input, x) = preceded(tag("{x="), u32_parser)(input)?;
    let (input, m) = preceded(tag(",m="), u32_parser)(input)?;
    let (input, a) = preceded(tag(",a="), u32_parser)(input)?;
    let (input, s) = preceded(tag(",s="), u32_parser)(input)?;
    let (input, _) = preceded(char_parser('}'), line_ending)(input)?;
    let part = Part { x, m, a, s };
    Ok((input, part))
}

pub fn parse_input(input: &'static str) -> Result<(Vec<Workflow>, Vec<Part>), AocError> {
    let (rest, (workflows, parts)) = separated_pair(
        many1(parse_workflow_line),
        line_ending,
        many1(parse_part_line),
    )(input)?;
    if rest.is_empty() {
        Ok((workflows, parts))
    } else {
        Err(AocError::ParseError(format!(
            "Parsing workflows and parts left rest: {}",
            rest
        )))
    }
}
