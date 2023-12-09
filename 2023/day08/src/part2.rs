use std::collections::HashMap;

use aoclib::AocError;

use crate::part1::{parse_input, Direction, Node};

fn is_starting_node(node_name: &str) -> bool {
    node_name.ends_with('A')
}

fn is_ending_node(node_name: &str) -> bool {
    node_name.ends_with('Z')
}

fn steps_to_next_end_node<'a>(
    start_node: &'a str,
    directions: &[Direction],
    nodes: &'a HashMap<String, Node>,
) -> Result<(i32, &'a str), AocError> {
    let mut steps_taken = 0;
    let mut current_node = start_node;
    let mut direction_iter = directions.iter().cycle();
    loop {
        let direction = direction_iter.next().unwrap();
        let next_node = nodes.get(current_node).ok_or(AocError::LogicError(format!(
            "Node {} not found",
            current_node
        )))?;
        current_node = next_node.get(*direction);
        steps_taken += 1;
        if is_ending_node(current_node) {
            break;
        }
    }
    Ok((steps_taken, current_node))
}

pub fn process(input: &'static str) -> Result<i32, AocError> {
    let (directions, nodes) = parse_input(input)?;
    let steps_map = nodes
        .keys()
        .filter_map(|node_name| {
            if is_ending_node(node_name) || is_starting_node(node_name) {
                let start = node_name.as_str();
                let (steps, end) =
                    steps_to_next_end_node(start, &directions, &nodes).unwrap_or((0, start));
                Some((start, (end, steps)))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    let mut current_nodes = nodes
        .keys()
        .filter_map(|node_name| {
            if is_starting_node(node_name) {
                let node_name_str = node_name.as_str();
                Some((
                    node_name_str,
                    steps_to_next_end_node(node_name_str, &directions, &nodes)
                        .unwrap_or((0, node_name_str))
                        .0,
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut steps_taken = current_nodes
        .iter()
        .map(|(_, steps)| *steps)
        .max()
        .unwrap_or(0);
    dbg!(&steps_map);
    while !current_nodes.iter().all(|(_, steps)| *steps == steps_taken) {
        let mut i = 0;
        while i < current_nodes.len() {
            let (node_name, steps) = current_nodes[i];
            if steps == steps_taken {
                i += 1;
                continue;
            }
            let (additional_steps, new_node_name) =
                steps_to_next_end_node(node_name, &directions, &nodes).unwrap_or((0, node_name));
            let new_steps = steps + additional_steps;
            current_nodes[i] = (new_node_name, new_steps);
            steps_taken = steps_taken.max(new_steps);
            i += 1;
        }
        if steps_taken > 1000000000 {
            return Err(AocError::LogicError(format!(
                "Steps taken {} is too high",
                steps_taken
            )));
        }
    }
    Ok(steps_taken)
}
