use std::env;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Filled(bool),
    Empty,
}

#[derive(Clone, Debug)]
struct Grid {
    size: usize,
    inner: Vec<Cell>,
}

impl Grid {
    fn new(size: usize) -> Self {
        assert_eq!(size % 2, 0);

        Grid {
            size,
            inner: vec![Cell::Empty; size * size],
        }
    }

    fn get(&self, x: usize, y: usize) -> Cell {
        let i = y * self.size + x;
        self.inner[i]
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        let i = y * self.size + x;
        self.inner[i] = Cell::Filled(value);
    }
}

fn main_cnf(filepath: PathBuf) {
    println!("Mode CNF");
    dbg!(filepath);
}

fn main_sol(filepath: PathBuf) {
    println!("Mode SOL");
    dbg!(filepath);
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
