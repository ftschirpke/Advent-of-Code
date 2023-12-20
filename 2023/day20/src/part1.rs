use std::collections::{HashMap, VecDeque};

use aoclib::AocError;

use crate::datastructures::{Module, LOW_PULSE};
use crate::parsing::parse_modules;

pub fn count_pulses(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut pulse_chunks_to_process: VecDeque<Vec<_>> = VecDeque::new();
    pulse_chunks_to_process.push_back(vec![(
        "button".to_string(),
        LOW_PULSE,
        "broadcaster".to_string(),
    )]);
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    loop {
        if pulse_chunks_to_process.is_empty() {
            break;
        }
        let chunk = pulse_chunks_to_process.pop_front().unwrap();

        let new_chunks = chunk
            .into_iter()
            .filter_map(|(from, pulse, to)| {
                if pulse == LOW_PULSE {
                    low_pulses += 1;
                } else {
                    high_pulses += 1;
                }
                if let Some(current_module) = modules.get_mut(&to) {
                    let pulse = current_module.receive_and_propagate_pulse(pulse, &from);
                    pulse.map(|pulse| {
                        current_module
                            .targets
                            .iter()
                            .map(|target| (to.clone(), pulse, target.clone()))
                            .collect()
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        pulse_chunks_to_process.extend(new_chunks);
    }
    (low_pulses, high_pulses)
}

pub fn process(input: &'static str) -> Result<usize, AocError> {
    let modules = parse_modules(input)?;
    let mut modules: HashMap<_, _> = modules
        .into_iter()
        .map(|module| (module.name.clone(), module))
        .collect();
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut seen_states = Vec::new();
    seen_states.push(
        modules
            .iter()
            .map(|(name, module)| (name.clone(), module.module_type.clone()))
            .collect::<HashMap<_, _>>(),
    );
    let mut pulse_counts = Vec::new();
    pulse_counts.push((low_pulses, high_pulses));
    for iteration in 1..=1000 {
        let (low, high) = count_pulses(&mut modules);
        low_pulses += low;
        high_pulses += high;
        let state = modules
            .iter()
            .map(|(name, module)| (name.clone(), module.module_type.clone()))
            .collect::<HashMap<_, _>>();
        if let Some(previous_occurence_iteration) = seen_states
            .iter()
            .position(|prev_state| *prev_state == state)
        {
            let loop_size = iteration - previous_occurence_iteration;
            let (prev_low, prev_high) = pulse_counts[previous_occurence_iteration];
            let loop_low = low_pulses - prev_low;
            let loop_high = high_pulses - prev_high;
            let remaining_iterations = 1000 - iteration;
            let loop_fits = remaining_iterations / loop_size;
            low_pulses += loop_low * loop_fits;
            high_pulses += loop_high * loop_fits;
            let loop_remainder = remaining_iterations % loop_size;
            let last_position = previous_occurence_iteration + loop_remainder;
            let (last_pos_low, last_pos_high) = pulse_counts[last_position];
            low_pulses += last_pos_low - prev_low;
            high_pulses += last_pos_high - prev_high;
            break;
        } else {
            seen_states.push(state);
            pulse_counts.push((low_pulses, high_pulses));
        }
    }
    Ok(low_pulses * high_pulses)
}
