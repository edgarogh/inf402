use crate::cnf::{CNFFile, Literal};
use crate::logic_utils::dnf_to_cnf;
use crate::Grid;
use std::iter::FromIterator;

/// Determine l'ensemble des combinaisons possible dans une ligne/colonne pour *nb_true*
pub fn combinations(nb_true: usize, size: usize) -> Vec<Vec<bool>> {
    if nb_true == size {
        return vec![vec![true; size as usize]];
    }

    if nb_true == 0 {
        return vec![vec![false; size as usize]];
    }

    let mut ret = Vec::new();

    for combination in combinations(nb_true, size - 1) {
        let mut new_combination = combination.clone();
        new_combination.push(false);
        ret.push(new_combination);
    }

    for combination in combinations(nb_true - 1, size - 1) {
        let mut new_combination = combination.clone();
        new_combination.push(true);
        ret.push(new_combination);
    }

    return ret;
}

pub fn write_rule_1<W>(out: &mut CNFFile<W>, grid: &Grid) {
    let combinations = combinaisons(grid.size / 2, grid.size);

    for a in 0..grid.size {
        let line_combinations: Vec<Vec<_>> = combinations
            .iter()
            .map(|combinations| {
                combinations
                    .iter()
                    .enumerate()
                    .map(|(x, vf)| Literal::new(x, a, *vf))
                    .collect()
            })
            .collect();

        let line_combinations_slice: Vec<_> = line_combinations
            .iter()
            .map(|combination| &combination[..])
            .collect();

        for clause in dnf_to_cnf(&line_combinations_slice[..]) {
            out.push(Vec::from_iter(clause));
        }
        let column_combinations: Vec<Vec<_>> = combinations
            .iter()
            .map(|combination| {
                combination
                    .iter()
                    .enumerate()
                    .map(|(y, vf)| Literal::new(a, y, *vf))
                    .collect()
            })
            .collect();

        let column_combinations_slice: Vec<_> = column_combinations
            .iter()
            .map(|combination| &combination[..])
            .collect();

        for clause in dnf_to_cnf(&column_combinations_slice[..]) {
            out.push(Vec::from_iter(clause));
        }
    }
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
