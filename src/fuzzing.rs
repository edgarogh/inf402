//! Fuzzing module using "20 Minute"'s Takuzu grids

use crate::cnf::CNFFile;
use crate::{Cell, Grid};
use std::convert::TryFrom;
use std::time::Instant;

fn parse_grid(grid: &str) -> Grid {
    Grid::try_from(
        grid.chars()
            .filter_map(|char| match char {
                '.' => Some(Cell::Empty),
                '0' => Some(Cell::Filled(false)),
                '1' => Some(Cell::Filled(true)),
                _ => None,
            })
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn fetch(i: usize) -> (Grid, Grid) {
    let url = format!(
        "https://rcijeux.fr/drupal_game/20minutes/takuzu/grids/{}.takj",
        i
    );
    let res = reqwest::blocking::get(url).expect("erreur de requête");
    let res = res.text().expect("erreur de lecture de la réponse");

    let (mut start, mut sol) = (None, None);

    for line in res.lines() {
        if let Some(grid) = line.strip_prefix("grille:\"") {
            start = Some(parse_grid(grid))
        }

        if let Some(grid) = line.strip_prefix("solution:\"") {
            sol = Some(parse_grid(grid))
        }
    }

    (start.unwrap(), sol.unwrap())
}

pub fn main_fuzzing() {
    for i in 1.. {
        eprintln!("================ {}", i);
        let (start, sol) = fetch(i);

        let mut output = CNFFile::new_varisat(&start);
        crate::rules::write_all(&mut output, &start);
        let output = output.into_varisat();

        let mut solver = varisat::Solver::new();
        solver.add_formula(&output);

        eprintln!("[varisat] solving");
        let instant_solving = Instant::now();
        assert!(solver.solve().unwrap());
        let model = solver.model().unwrap();
        eprintln!("\\ DONE ({:?})", instant_solving.elapsed());

        let end = Grid::try_from(
            model
                .into_iter()
                .map(|lit| Cell::Filled(lit.is_positive()))
                .collect::<Vec<_>>(),
        )
        .unwrap();

        assert_eq!(end, sol);
    }
}
