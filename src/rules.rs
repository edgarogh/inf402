use crate::cnf::{CNFFile, Literal};
use crate::Grid;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

fn dnf_to_cnf<F: Copy + Eq + Hash>(dnf: &[&[F]]) -> Vec<HashSet<F>> {
    match dnf {
        [] => unimplemented!(),
        [con_clause] => con_clause.into_iter().map(|lit| HashSet::from_iter(vec![lit].into_iter().copied())).collect(),
        [start @ .., con_clause] => {
            let cnf2 = dnf_to_cnf(start);

            cnf2.into_iter()
                .map(|clause| {
                    con_clause
                        .into_iter()
                        .map(|lit| HashSet::from_iter(vec![lit].into_iter().copied()))
                        .map(|mut new_clause| {
                            new_clause.extend(clause.iter().cloned());
                            new_clause
                        })
                        .collect::<Vec<_>>() // TODO
                })
                .flatten()
                .collect()
        }
    }
}

pub fn write_rule_1<W>(out: &mut CNFFile<W>, grid: &Grid) {
    // TODO
}

pub fn write_rule_2<W>(out: &mut CNFFile<W>, grid: &Grid) {
    // TODO
}

pub fn write_rule_3<W>(out: &mut CNFFile<W>, grid: &Grid) {
    // exemple:
    out.push(vec![Literal::new(0, 1, true)])
    // TODO
}

pub fn write_all<W>(out: &mut CNFFile<W>, grid: &Grid) {
    write_rule_1(out, grid);
    write_rule_2(out, grid);
    write_rule_3(out, grid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dnf_to_cnf() {
        let (a, b, c, d) = ("a", "b", "c", "d");

        eprintln!("{:#?}", dnf_to_cnf(&[&[a, b, c], &[c, d]]))
    }
}
