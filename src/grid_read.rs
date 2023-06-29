use crate::Grid;
use std::fs;
use std::num::NonZeroUsize;
use std::path::PathBuf;

pub fn file_read(filepath: PathBuf) -> String {
    fs::read_to_string(filepath).expect("Problème lors de la lecture de la grille")
}

pub fn size(content: &str) -> Option<NonZeroUsize> {
    content.lines().next().and_then(|l| l.parse().ok())
}

pub fn fill_grid_from_file(grid: &mut Grid, content: &str) {
    let bytes = content.as_bytes();
    let mut x = 0;
    let mut y = 0;
    let mut s = false;

    for &element in bytes.iter().filter(|c| **c != b'\r') {
        if element == b'\n' {
            if !s {
                s = true;
            }
        } else if s {
            if x < grid.size && y < grid.size {
                if element == b'1' {
                    grid.set(x, y, true);
                } else if element == b'0' {
                    grid.set(x, y, false);
                }
            }
            x += 1;
            if x == grid.size {
                y += 1;
                x = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Cell;

    const BASIC: &str = "2\n01\n11\n";

    #[test]
    fn basic() {
        let grid = {
            let mut grid = Grid::new(size(BASIC).unwrap().get());
            fill_grid_from_file(&mut grid, BASIC);
            grid
        };

        assert_eq!(grid.size, 2);
        assert_eq!(grid.get(0, 0), Cell::Filled(false));
        assert_eq!(grid.get(0, 1), Cell::Filled(true));
        assert_eq!(grid.get(1, 0), Cell::Filled(true));
        assert_eq!(grid.get(1, 1), Cell::Filled(true));
    }
}
