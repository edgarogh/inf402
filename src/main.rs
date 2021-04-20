use std::env;

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

fn main() {
    let args: Vec<String> = env::args().collect();

    let (mode, filename) = match args.as_slice() {
        [_, mode, filename] if mode == "sol" => (mode, filename),
        [_, mode, filename] if mode == "cnf" => (mode, filename),
        [_, mode, filename] => {
            println!("Mode inconnu (sol ou cnf)");
            return;
        }
        _ => {
            println!("Mauvais nombre d'arguments");
            return;
        }
    };

    println!("Fichier entrée : {} avec le mode {}", filename, mode);
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
