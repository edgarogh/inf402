#[macro_use]
extern crate rocket;

use std::convert::TryFrom;
use std::env;
use std::fmt::{Display, Formatter};

mod cnf;
mod grid_read;
mod logic_utils;
mod rules;
mod web;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cell {
    Filled(bool),
    Empty,
}

impl Cell {
    pub fn to_char(self) -> char {
        match self {
            Self::Filled(false) => '0',
            Self::Filled(true) => '1',
            Self::Empty => '.',
        }
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    size: usize,
    inner: Vec<Cell>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        assert_ne!(size, 0);
        assert_eq!(size % 2, 0);

        Grid {
            size,
            inner: vec![Cell::Empty; size * size],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        let i = y * self.size + x;
        self.inner[i]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        let i = y * self.size + x;
        self.inner[i] = Cell::Filled(value);
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size {
            for x in 0..self.size {
                let c = self.get(x, y);
                match c {
                    Cell::Filled(true) => write!(f, "1")?,
                    Cell::Filled(false) => write!(f, "0")?,
                    Cell::Empty => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

/// Construit une grille à partir d'une liste de cellules, de façon safe (vérification de la taille)
impl TryFrom<Vec<Cell>> for Grid {
    type Error = ();

    fn try_from(value: Vec<Cell>) -> Result<Self, Self::Error> {
        /// Racine carrée entière "stricte", renvoie `None` si le nombre n'en a pas
        fn int_sqrt(n: usize) -> Option<usize> {
            match n {
                // Les premiers cas sont là pour des raisons de performance
                4 => Some(2),
                16 => Some(4),
                36 => Some(6),
                64 => Some(8),
                // Cas général, recourt à des flottants
                n => {
                    let sqrt = (n as f64).sqrt() as usize;
                    if sqrt.pow(2) == n {
                        Some(sqrt)
                    } else {
                        None
                    }
                }
            }
        }

        if let Some(size) = int_sqrt(value.len()) {
            Ok(Grid { size, inner: value })
        } else {
            Err(())
        }
    }
}

#[rocket::launch]
fn launch() -> _ {
    let args: Vec<String> = env::args().collect();

    web::main_rocket(args.get(1).cloned().unwrap_or_default())
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_get_set() {
        let mut g = Grid::new(6);
        g.set(0, 1, true);
        g.set(0, 0, false);
        assert_eq!(g.get(0, 1), Cell::Filled(true));
        assert_eq!(g.get(0, 0), Cell::Filled(false));
        assert_eq!(g.get(0, 0), Cell::Filled(false));
    }
}
