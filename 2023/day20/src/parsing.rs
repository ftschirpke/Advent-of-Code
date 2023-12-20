use std::collections::HashMap;

use aoclib::AocError;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, one_of},
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

use crate::datastructures::{Module, ModuleType};

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModuleData {
    name: String,
    module_type_symbol: char,
    targets: Vec<String>,
}

fn parse_module_data(input: &str) -> IResult<&str, ModuleData> {
    let (input, (symbol, name)) = tuple((one_of("b%&"), alpha1))(input)?;
    let (input, targets) = preceded(tag(" -> "), separated_list1(tag(", "), alpha1))(input)?;
    let (input, _) = line_ending(input)?;
    let mut targets: Vec<_> = targets.into_iter().map(|s| s.to_string()).collect();
    targets.sort();
    let module_data = ModuleData {
        name: name.to_string(),
        module_type_symbol: symbol,
        targets: targets.into_iter().map(|s| s.to_string()).collect(),
    };
    Ok((input, module_data))
}

pub fn parse_modules(input: &'static str) -> Result<Vec<Module>, AocError> {
    let (rest, modules_data) = many1(parse_module_data)(input)?;
    if !rest.is_empty() {
        return Err(AocError::ParseError(format!(
            "Parsing modules left rest: {}",
            rest
        )));
    }
    let mut incoming: HashMap<String, Vec<String>> = HashMap::new();
    modules_data.iter().for_each(|data| {
        data.targets.iter().for_each(|target| {
            incoming
                .entry(target.to_string())
                .or_default()
                .push(data.name.to_string());
        })
    });
    let modules = modules_data
        .into_iter()
        .map(|data| match data.module_type_symbol {
            'b' => Module::new(
                format!("b{}", data.name),
                ModuleType::new_broadcast(),
                data.targets,
            ),
            '%' => Module::new(data.name, ModuleType::new_flipflop(), data.targets),
            '&' => {
                let conj_type = ModuleType::new_conjunction(incoming[&data.name].clone());
                Module::new(data.name, conj_type, data.targets)
            }
            _ => unreachable!("Invalid module type symbol should have been caught by parser"),
        })
        .collect();
    Ok(modules)
}
