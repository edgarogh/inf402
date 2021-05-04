use crate::{Cell, Grid};
use std::collections::HashSet;
use std::convert::TryInto;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::num::{NonZeroIsize, NonZeroUsize};
use varisat::{CnfFormula, Lit};

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

    fn into_numeric(self, grid_size: usize) -> NonZeroIsize {
        let sign = if self.negated { -1 } else { 1 };

        let index = (self.x + (self.y * grid_size)) as isize;

        unsafe { NonZeroIsize::new_unchecked((1 + index) * sign) }
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
pub struct CNFFile<F = BufWriter<File>> {
    initial: HashSet<Literal>,
    grid_size: NonZeroUsize,
    writer: Option<F>,
    clauses: Vec<Vec<Literal>>,
}

impl<F> CNFFile<F> {
    pub fn push(&mut self, clause: Vec<Literal>) {
        self.push_multiple(std::iter::once(clause));
    }

    pub fn push_multiple(&mut self, new_clauses: impl IntoIterator<Item = Vec<Literal>>) {
        let new_clauses = new_clauses.into_iter();

        let initial = &self.initial;

        self.clauses
            .extend(new_clauses.filter(|c| !c.iter().any(|l| initial.contains(l))));
    }
}

impl<F: Write> CNFFile<F> {
    pub fn new(grid: &Grid, writer: F) -> Self {
        let initial = grid.to_literals();

        Self {
            grid_size: grid.size.try_into().unwrap(),
            writer: Some(writer),
            clauses: initial.iter().copied().map(|l| vec![l]).collect(),
            initial,
        }
    }

    /// Enregistre le fichier CNF, détruit le `CNFFile` et renvoie le `Write` interieur
    pub fn save(self) -> std::io::Result<F> {
        let Self {
            grid_size,
            clauses,
            mut writer,
            ..
        } = self;

        let mut writer = writer.take().unwrap();

        writeln!(
            &mut writer,
            "p cnf {} {}",
            grid_size.get().pow(2),
            clauses.len()
        )?;

        for clause in clauses {
            for literal in clause {
                write!(&mut writer, "{} ", literal.into_numeric(grid_size.get()))?;
            }
            writeln!(&mut writer, "0")?;
        }

        Ok(writer)
    }
}

/// For varisat output
impl CNFFile<()> {
    pub fn new_varisat(grid: &Grid) -> Self {
        let initial = grid.to_literals();

        Self {
            grid_size: grid.size.try_into().unwrap(),
            writer: None,
            clauses: initial.iter().copied().map(|l| vec![l]).collect(),
            initial,
        }
    }

    pub fn into_varisat(self) -> CnfFormula {
        let Self {
            grid_size, clauses, ..
        } = self;

        CnfFormula::from(clauses.into_iter().map(|clause| {
            clause
                .into_iter()
                .map(|lit| Lit::from_dimacs(lit.into_numeric(grid_size.get()).get()))
                .collect::<Vec<_>>()
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut cnf = CNFFile::new(
            // La grille n'a pas d'importance ici, on en créée une temporaire juste pour cet appel
            &Grid::new(2),
            // Plutôt que d'écrire vers un fichier, on écrit vers un tableau d'octets en mémoire;
            // c'est plus facile à tester. On peut faire ça car `CNFFile` est générique et accepte
            // n'importe quel type implémentant `Write`, donc `Vec<u8>` et `File` font partie.
            Vec::new(),
        );

        cnf.push(vec![Literal::new(0, 1, true), Literal::new(1, 1, false)]);
        cnf.push(vec![Literal::new(0, 0, true), Literal::new(1, 1, true)]);

        let out = String::from_utf8(cnf.save().unwrap()).unwrap();

        assert_eq!(out, "p cnf 4 2\n3 -4 0\n1 4 0\n")
    }
}
