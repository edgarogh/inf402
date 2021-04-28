mod cnf;
mod grid_read;
mod rules;
mod sat;

use crate::cnf::CNFFile;
use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cell {
    Filled(bool),
    Empty,
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

    pub fn print(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                let c = self.get(x, y);
                match c {
                    Cell::Filled(true) => print!("1"),
                    Cell::Filled(false) => print!("0"),
                    Cell::Empty => print!("."),
                }
            }
            println!();
        }
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

fn main_cnf(filepath: PathBuf) {
    eprintln!("lecture de la grille {:?}", filepath);
    let content: String = grid_read::file_read(filepath);
    let grid_size: usize = grid_read::size(&content)
        .trim()
        .parse()
        .expect("Taille incorrecte dans le fichier");

    let mut grid: Grid = Grid::new(grid_size);
    grid_read::fill_grid_from_file(&mut grid, &content);

    let stdout = std::io::stdout();
    let mut output = CNFFile::new(&grid, BufWriter::new(stdout.lock()));
    rules::write_all(&mut output, &grid);
    output.save().unwrap();
}

fn main_sol(filepath: PathBuf) {
    eprintln!("lecture du fichier de résultats: {:?}", filepath);
    let file = std::io::BufReader::new(File::open(filepath).unwrap());
    let grid = sat::read_sat_file(file).unwrap();
    eprintln!("grille: ");
    grid.print();
}

/// exe: nom de l'exécutable pour le message d'aide
fn help(exe: &str) {
    eprintln!(
        "Usage: `{0} sol <fichier.takuzu>` ou `{0} cnf <fichier.resultat>`",
        exe,
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_, mode, filename] if mode == "sol" => main_sol(filename.into()),
        [_, mode, filename] if mode == "cnf" => main_cnf(filename.into()),
        [exe, _, _] => {
            eprintln!("Mode inconnu.");
            help(exe);
            return;
        }
        [exe] => {
            eprintln!("Deux arguments attendus.");
            help(exe);
            return;
        }
        _ => {
            eprintln!("Deux arguments attendus.");
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::combinaisons;

    #[test]
    fn grid_get_set() {
        let mut g = Grid::new(6);
        g.set(0, 1, true);
        g.set(0, 0, false);
        assert_eq!(g.get(0, 1), Cell::Filled(true));
        assert_eq!(g.get(0, 0), Cell::Filled(false));
        assert_eq!(g.get(0, 0), Cell::Filled(false));
        let ret = combinaisons(2, 4);
        println!("{:?}", ret);
    }
}
