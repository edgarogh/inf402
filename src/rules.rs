use crate::cnf::{CNFFile, Literal};
use crate::logic_utils;
use crate::Grid;

pub fn combinaisons(nb_true: usize, size: usize) -> Vec<Vec<bool>> {
    if nb_true == size {
        return vec![vec![true; size as usize]];
    }
    if nb_true == 0 {
        return vec![vec![false; size as usize]];
    }
    let mut ret = Vec::new();
    for combinaison in combinaisons(nb_true, size - 1) {
        let mut nouvelle_combinaison = combinaison.clone();
        nouvelle_combinaison.push(false);
        ret.push(nouvelle_combinaison);
    }
    for combinaison in combinaisons(nb_true - 1, size - 1) {
        let mut nouvelle_combinaison = combinaison.clone();
        nouvelle_combinaison.push(true);
        ret.push(nouvelle_combinaison);
    }
    return ret;
}

pub fn write_rule_1<W>(out: &mut CNFFile<W>, grid: &Grid) {}

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
