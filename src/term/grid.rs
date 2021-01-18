use std::convert::TryFrom;

use crate::term::cell::Cell;

#[derive(Copy, Clone, Debug)]
pub struct GridDimensions {
    pub columns: u16,
    pub lines: u16,
}

impl GridDimensions {
    pub fn from_window_size(width: u32, height: u32) -> Self {
        let width = u16::try_from(width).unwrap();
        let height = u16::try_from(height).unwrap();
        Self {
            columns: width / Cell::CELL_WIDTH,
            lines: height / Cell::CELL_HEIGHT,
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    pub dimensions: GridDimensions,
    pub content: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(dimensions: &GridDimensions) -> Self {
        Grid {
            dimensions: *dimensions,
            content: vec![
                vec![Cell::new(); usize::from(dimensions.columns)];
                usize::from(dimensions.lines)
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_window_size() {
        let dimensions = GridDimensions::from_window_size(100, 200);
        assert_eq!(dimensions.columns, 10);
        assert_eq!(dimensions.lines, 10);
    }
}
