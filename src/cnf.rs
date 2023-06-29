use crate::{Cell, Grid};
use std::borrow::Borrow;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fmt::Display;
use std::num::NonZeroUsize;
use varisat::solver::SolverError;
use varisat::{CnfFormula, ExtendFormula, Lit, Solver};

/// Takuzu-focused literal representation, using actual coordinates
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Literal {
    x: usize,
    y: usize,

    /// Is the literal a negated atom
    negated: bool,
}

impl Literal {
    pub fn new(x: usize, y: usize, positive: bool) -> Self {
        Self {
            x,
            y,
            negated: !positive,
        }
    }

    fn into_numeric(self, grid_size: usize) -> isize {
        let sign = if self.negated { -1 } else { 1 };

        let index = (self.x + (self.y * grid_size)) as isize;

        (1 + index) * sign
    }

    pub fn into_lit(self, grid_size: usize) -> varisat::Lit {
        Lit::from_dimacs(self.into_numeric(grid_size))
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        if self.negated {
            f.write_char('¬')?;
        }

        write!(f, "({}, {})", self.x, self.y)
    }
}

fn literal_to_lits(
    literals: impl IntoIterator<Item = impl Borrow<Literal>>,
    grid_size: usize,
) -> Vec<Lit> {
    literals
        .into_iter()
        .map(|l| l.borrow().into_lit(grid_size))
        .collect()
}

impl Grid {
    pub fn to_literals(&self) -> HashSet<Literal> {
        self.inner
            .iter()
            .enumerate()
            .filter_map(|(index, cell)| match *cell {
                Cell::Empty => None,
                Cell::Filled(p) => Some(Literal::new(index % self.size, index / self.size, p)),
            })
            .collect()
    }
}

/// Un fichier CNF pouvant être produit par ce logiciel
pub struct CNFFile<'a> {
    grid_size: NonZeroUsize,
    // formula: CnfFormula,
    solver: Solver<'a>,
}

impl CNFFile<'_> {
    pub fn new_varisat(grid: &Grid) -> Self {
        let initial = grid.to_literals();

        let solver = {
            let mut s = Solver::new();
            s.assume(
                &initial
                    .iter()
                    .copied()
                    .map(|l| l.into_lit(grid.size))
                    .collect::<Vec<_>>(),
            );
            s
        };

        Self {
            grid_size: grid.size.try_into().unwrap(),
            // formula,
            solver,
        }
    }

    pub fn push(&mut self, clause: Vec<Literal>) {
        self.solver
            .add_clause(&literal_to_lits(clause, self.grid_size.get()));
    }

    pub fn push_multiple(&mut self, new_clauses: impl IntoIterator<Item = Vec<Literal>>) {
        self.solver.add_formula(&CnfFormula::from(
            new_clauses
                .into_iter()
                .map(|clause| literal_to_lits(clause, self.grid_size.get())),
        ));
    }

    pub fn solve(&mut self) -> Result<bool, SolverError> {
        self.solver.solve()
    }

    pub fn model(self) -> Option<Vec<Lit>> {
        self.solver.model()
    }
}
