use aoclib::AocError;
use nom::{
    branch::alt,
    character::complete::{char as char_parser, line_ending},
    combinator::value,
    multi::many1,
    sequence::terminated,
    IResult,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rock {
    Empty,
    Rolling,
    Stationary,
}

#[derive(Debug)]
pub struct Grid {
    pub rock_columns: Vec<Vec<Rock>>,
    pub cols: usize,
    pub rows: usize,
}

impl Grid {
    pub fn new(rock_rows: Vec<Vec<Rock>>) -> Result<Self, AocError> {
        let rows = rock_rows.len();
        let cols = rock_rows[0].len();
        let mut rock_columns = vec![vec![Rock::Empty; rows]; cols];
        rock_rows
            .into_iter()
            .enumerate()
            .try_for_each(|(row_idx, row)| {
                if row.len() != cols {
                    return Err(AocError::ParseError(
                        "All rows must have the same length".to_string(),
                    ));
                }
                row.into_iter()
                    .enumerate()
                    .filter(|(_, rock)| *rock != Rock::Empty)
                    .for_each(|(col_idx, rock)| rock_columns[col_idx][row_idx] = rock);
                Ok::<(), AocError>(())
            })?;
        let grid = Grid {
            rock_columns,
            cols,
            rows,
        };
        Ok(grid)
    }
}

fn parse_rock(input: &str) -> IResult<&str, Rock> {
    alt((
        value(Rock::Empty, char_parser('.')),
        value(Rock::Rolling, char_parser('O')),
        value(Rock::Stationary, char_parser('#')),
    ))(input)
}

fn parse_rock_row(input: &str) -> IResult<&str, Vec<Rock>> {
    terminated(many1(parse_rock), line_ending)(input)
}

pub fn parse_grid(input: &str) -> Result<Grid, AocError> {
    let (rest, rock_rows) = many1(parse_rock_row)(input)?;
    if rest.is_empty() {
        Grid::new(rock_rows)
    } else {
        Err(AocError::ParseError(format!(
            "Parsing the grid of rocks left rest: {}",
            rest
        )))
    }
}

pub fn process(input: &'static str) -> Result<i32, AocError> {
    let grid = parse_grid(input)?;
    let pressure_sum = grid.rock_columns.into_iter().fold(0i32, |grid_acc, rocks| {
        let mut next_pressure_value = grid.rows as i32;
        let column_pressure = rocks
            .into_iter()
            .enumerate()
            .fold(0i32, |col_acc, (i, rock)| match rock {
                Rock::Empty => col_acc,
                Rock::Stationary => {
                    next_pressure_value = (grid.rows - i - 1) as i32;
                    col_acc
                }
                Rock::Rolling => {
                    let val = next_pressure_value;
                    next_pressure_value -= 1;
                    col_acc + val
                }
            });
        grid_acc + column_pressure
    });
    Ok(pressure_sum)
}
