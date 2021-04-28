use crate::cnf::{CNFFile, Literal};
use crate::logic_utils::dnf_to_cnf;
use crate::Grid;
use std::iter::FromIterator;

/// Determine l'ensemble des combinaisons possible dans une ligne/colonne pour *nb_true*
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

pub fn write_rule_1<W>(out: &mut CNFFile<W>, grid: &Grid) {
    let combinaisons = combinaisons(grid.size / 2, grid.size);
    println!("{:?}", combinaisons);

    for a in 0..grid.size {
        let ligne_combinaisons: Vec<Vec<_>> = combinaisons
            .iter()
            .map(|combinaison| {
                combinaison
                    .iter()
                    .enumerate()
                    .map(|(x, vf)| Literal::new(x, a, *vf))
                    .collect()
            })
            .collect();

        let ligne_combinaisons_slice: Vec<_> = ligne_combinaisons
            .iter()
            .map(|sous_vec| &sous_vec[..])
            .collect();

        println!("{:?}", ligne_combinaisons_slice);

        for clause in dnf_to_cnf(&ligne_combinaisons_slice[..]) {
            out.push(Vec::from_iter(clause));
        }
        let colonne_combinaisons: Vec<Vec<_>> = combinaisons
            .iter()
            .map(|combinaison| {
                combinaison
                    .iter()
                    .enumerate()
                    .map(|(y, vf)| Literal::new(a, y, *vf))
                    .collect()
            })
            .collect();

        let colonne_combinaisons_slice: Vec<_> = colonne_combinaisons
            .iter()
            .map(|sous_vec| &sous_vec[..])
            .collect();

        for clause in dnf_to_cnf(&colonne_combinaisons_slice[..]) {
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
