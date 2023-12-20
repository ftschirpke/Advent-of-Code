use std::collections::HashMap;

pub const LOW_PULSE: bool = false;
pub const HIGH_PULSE: bool = true;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleType {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

impl ModuleType {
    pub fn new_broadcast() -> Self {
        ModuleType::Broadcast
    }

    pub fn new_flipflop() -> Self {
        ModuleType::FlipFlop(false)
    }

    pub fn new_conjunction(incoming: Vec<String>) -> Self {
        let remember = incoming
            .into_iter()
            .map(|s| (s, LOW_PULSE))
            .collect::<HashMap<String, bool>>();
        ModuleType::Conjunction(remember)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name: String,
    pub module_type: ModuleType,
    pub targets: Vec<String>,
}

impl Module {
    pub fn new(name: String, module_type: ModuleType, targets: Vec<String>) -> Self {
        Module {
            name: name.to_string(),
            module_type,
            targets,
        }
    }

    pub fn receive_and_propagate_pulse(&mut self, pulse: bool, from: &str) -> Option<bool> {
        match &mut self.module_type {
            ModuleType::Broadcast => Some(pulse),
            ModuleType::FlipFlop(ref mut state) => {
                if pulse == LOW_PULSE {
                    *state = !*state;
                    Some(*state)
                } else {
                    None
                }
            }
            ModuleType::Conjunction(remember) => {
                *remember.entry(from.to_string()).or_insert(LOW_PULSE) = pulse;
                if remember.values().all(|&v| v == HIGH_PULSE) {
                    Some(LOW_PULSE)
                } else {
                    Some(HIGH_PULSE)
                }
            }
        }
    }
}
