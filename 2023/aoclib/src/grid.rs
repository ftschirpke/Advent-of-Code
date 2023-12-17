use crate::AocError;

use nom::{character::complete::line_ending, multi::many1, sequence::terminated, IResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T>
where
    T: Clone + Eq,
    for<'a> &'a T: Into<char>,
{
    elements: Vec<T>,
    pub cols: usize,
    pub rows: usize,
}

impl<T> Grid<T>
where
    T: Clone + Eq,
    for<'a> &'a T: Into<char>,
{
    pub fn new(element_rows: Vec<Vec<T>>) -> Result<Self, AocError> {
        let rows = element_rows.len();
        let cols = element_rows[0].len();
        element_rows.iter().try_for_each(|row| {
            if row.len() != cols {
                return Err(AocError::ParseError(
                    "All rows must have the same length to create a grid".to_string(),
                ));
            }
            Ok::<(), AocError>(())
        })?;
        let elements = element_rows.into_iter().flatten().collect();
        let grid = Grid {
            elements,
            cols,
            rows,
        };
        Ok(grid)
    }

    pub fn parse_from(
        input: &str,
        element_parser: impl Fn(&str) -> IResult<&str, T>,
    ) -> Result<Self, AocError> {
        let (rest, element_rows) = many1(terminated(many1(element_parser), line_ending))(input)?;
        if rest.is_empty() {
            Self::new(element_rows)
        } else {
            Err(AocError::ParseError(format!(
                "Parsing grid left rest: {}",
                rest
            )))
        }
    }

    pub fn column(&self, col: usize) -> impl DoubleEndedIterator<Item = &T> {
        self.elements.iter().skip(col).step_by(self.cols)
    }

    pub fn row(&self, row: usize) -> impl DoubleEndedIterator<Item = &T> {
        self.elements.iter().skip(row * self.cols).take(self.cols)
    }

    pub fn column_mut(&mut self, col: usize) -> impl DoubleEndedIterator<Item = &mut T> {
        self.elements.iter_mut().skip(col).step_by(self.cols)
    }

    pub fn row_mut(&mut self, row: usize) -> impl DoubleEndedIterator<Item = &mut T> {
        self.elements
            .iter_mut()
            .skip(row * self.cols)
            .take(self.cols)
    }

    pub fn print(&self) {
        self.elements.chunks(self.cols).for_each(|row| {
            println!(
                "{}",
                row.iter().map(|element| element.into()).collect::<String>()
            )
        });
        println!();
    }
}
