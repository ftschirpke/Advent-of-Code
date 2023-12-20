use std::collections::{HashMap, VecDeque};

use aoclib::AocError;
use num::integer::gcd;

use crate::datastructures::{Module, HIGH_PULSE, LOW_PULSE};
use crate::parsing::parse_modules;

pub fn press_button_and_abort_on_rx_low(
    modules: &mut HashMap<String, Module>,
    key_module_name: &str,
    channeling_module_counts: &mut HashMap<String, usize>,
    channeling_cycle_counts: &mut HashMap<String, usize>,
    iteration: usize,
) -> bool {
    let mut pulse_chunks_to_process: VecDeque<Vec<_>> = VecDeque::new();
    pulse_chunks_to_process.push_back(vec![(
        "button".to_string(),
        LOW_PULSE,
        "broadcaster".to_string(),
    )]);
    loop {
        if pulse_chunks_to_process.is_empty() {
            break;
        }
        let chunk = pulse_chunks_to_process.pop_front().unwrap();

        let mut new_chunks = Vec::new();
        for (from, pulse, to) in chunk {
            if pulse == HIGH_PULSE && to == key_module_name {
                channeling_module_counts
                    .entry(from.clone())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                channeling_cycle_counts
                    .entry(from.clone())
                    .or_insert(iteration);

                if channeling_module_counts.values().all(|&count| count >= 1) {
                    return true;
                }
            }
            if let Some(current_module) = modules.get_mut(&to) {
                let pulse = current_module.receive_and_propagate_pulse(pulse, &from);
                if let Some(pulse) = pulse {
                    new_chunks.push(
                        current_module
                            .targets
                            .iter()
                            .map(|target| (to.clone(), pulse, target.clone()))
                            .collect(),
                    );
                }
            }
        }
        pulse_chunks_to_process.extend(new_chunks);
    }
    false
}

pub fn process(input: &'static str) -> Result<usize, AocError> {
    let modules = parse_modules(input)?;
    let mut modules: HashMap<_, _> = modules
        .into_iter()
        .map(|module| (module.name.clone(), module))
        .collect();

    let channel_into_rx = modules
        .values()
        .filter(|module| module.targets.contains(&"rx".to_string()))
        .cloned()
        .collect::<Vec<_>>();
    let key_module = match &channel_into_rx[..] {
        [single] => single,
        _ => {
            return Err(AocError::LogicError(
                "Expected single module with rx as target".to_string(),
            ))
        }
    };

    let mut channeling_module_counts: HashMap<String, usize> = modules
        .iter()
        .filter_map(|(name, module)| {
            if module.targets.contains(&key_module.name) {
                Some((name.clone(), 0))
            } else {
                None
            }
        })
        .collect();
    let mut channeling_cycle_counts: HashMap<String, usize> = HashMap::new();

    for iteration in 1.. {
        if press_button_and_abort_on_rx_low(
            &mut modules,
            &key_module.name,
            &mut channeling_module_counts,
            &mut channeling_cycle_counts,
            iteration,
        ) {
            let rx_iteration = channeling_cycle_counts
                .values()
                .fold(1usize, |acc, &cycle_count| {
                    acc * cycle_count / gcd(acc, cycle_count)
                });
            return Ok(rx_iteration);
        }
    }
    unreachable!()
}

