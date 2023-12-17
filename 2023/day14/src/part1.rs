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

impl Rock {
    pub fn to_char(&self) -> char {
        match *self {
            Rock::Empty => '.',
            Rock::Rolling => 'O',
            Rock::Stationary => '#',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    rocks: Vec<Rock>,
    pub cols: usize,
    pub rows: usize,
}

impl Grid {
    pub fn new(rock_rows: Vec<Vec<Rock>>) -> Result<Self, AocError> {
        let rows = rock_rows.len();
        let cols = rock_rows[0].len();
        rock_rows.iter().try_for_each(|row| {
            if row.len() != cols {
                return Err(AocError::ParseError(
                    "All rows must have the same length".to_string(),
                ));
            }
            Ok::<(), AocError>(())
        })?;
        let rocks = rock_rows.into_iter().flatten().collect();
        let grid = Grid { rocks, cols, rows };
        Ok(grid)
    }

    pub fn column(&self, col: usize) -> impl DoubleEndedIterator<Item = &Rock> {
        self.rocks.iter().skip(col).step_by(self.cols)
    }

    pub fn row(&self, row: usize) -> impl DoubleEndedIterator<Item = &Rock> {
        self.rocks.iter().skip(row * self.cols).take(self.cols)
    }

    pub fn column_mut(&mut self, col: usize) -> impl DoubleEndedIterator<Item = &mut Rock> {
        self.rocks.iter_mut().skip(col).step_by(self.cols)
    }

    pub fn row_mut(&mut self, row: usize) -> impl DoubleEndedIterator<Item = &mut Rock> {
        self.rocks.iter_mut().skip(row * self.cols).take(self.cols)
    }

    pub fn print(&self) {
        self.rocks.chunks(self.cols).for_each(|row| {
            println!(
                "{}",
                row.iter().map(|rock| rock.to_char()).collect::<String>()
            )
        });
        println!();
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
    let total_pressure = (0..grid.cols)
        .map(|col_num| {
            let mut next_pressure_value = grid.rows as i32;
            let column_pressure: i32 = grid
                .column(col_num)
                .enumerate()
                .filter_map(|(i, rock)| match rock {
                    Rock::Empty => None,
                    Rock::Stationary => {
                        next_pressure_value = (grid.rows - i - 1) as i32;
                        None
                    }
                    Rock::Rolling => {
                        let val = next_pressure_value;
                        next_pressure_value -= 1;
                        Some(val)
                    }
                })
                .sum();
            column_pressure
        })
        .sum();
    Ok(total_pressure)
}
