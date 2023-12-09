use std::collections::HashMap;

use aoclib::AocError;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, newline};
use nom::multi::fold_many1;
use nom::sequence::{delimited, separated_pair};
use nom::{
    branch::alt, character::complete::char as char_parser, combinator::value, multi::many1, IResult,
};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Node {
    left: String,
    right: String,
}

impl Node {
    pub fn new(left: String, right: String) -> Self {
        Self { left, right }
    }

    pub fn get(&self, direction: Direction) -> &str {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    let left_parser = value(Direction::Left, char_parser('L'));
    let right_parser = value(Direction::Right, char_parser('R'));
    let (input, directions) = many1(alt((left_parser, right_parser)))(input)?;
    let (input, _) = newline(input)?;
    Ok((input, directions))
}

fn parse_node(input: &str) -> IResult<&str, (String, Node)> {
    let left_right_parser = separated_pair(alphanumeric1, tag(", "), alphanumeric1);
    let (input, (node_name, (left, right))) = separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(char_parser('('), left_right_parser, char_parser(')')),
    )(input)?;
    let (input, _) = newline(input)?;
    Ok((
        input,
        (
            node_name.to_string(),
            Node::new(left.to_string(), right.to_string()),
        ),
    ))
}

fn parse_nodes(input: &str) -> IResult<&str, HashMap<String, Node>> {
    fold_many1(parse_node, HashMap::new, |mut acc, (node_name, node)| {
        acc.insert(node_name, node);
        acc
    })(input)
}

pub fn parse_input(input: &str) -> Result<(Vec<Direction>, HashMap<String, Node>), AocError> {
    let (input, directions) = parse_directions(input)?;
    let (input, _) = newline(input)?;
    let (rest, nodes) = parse_nodes(input)?;
    if rest.is_empty() {
        Ok((directions, nodes))
    } else {
        Err(AocError::ParseError(format!(
            "Parsing of directions and nodes left rest: {}",
            rest
        )))
    }
}

const START_NODE: &str = "AAA";
const END_NODE: &str = "ZZZ";

pub fn process(input: &'static str) -> Result<i32, AocError> {
    let (directions, nodes) = parse_input(input)?;
    let mut steps_taken = 0;
    let mut current_node = START_NODE;
    let mut direction_iter = directions.iter().cycle();
    while current_node != END_NODE {
        let direction = direction_iter.next().unwrap();
        let next_node = nodes.get(current_node).ok_or(AocError::LogicError(format!(
            "Node {} not found",
            current_node
        )))?;
        current_node = next_node.get(*direction);
        steps_taken += 1;
    }
    Ok(steps_taken)
}
