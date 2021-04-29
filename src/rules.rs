use crate::cnf::{CNFFile, Literal};
use crate::logic_utils::dnf_to_cnf;
use crate::Grid;
use std::iter::FromIterator;
use std::time::Instant;

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
    let combinations = combinations(grid.size / 2, grid.size);

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
    for x in 0..grid.size - 1 {
        for y in 0..grid.size - 3 {
            out.push(vec![
                Literal::new(x, y, true),
                Literal::new(x, y + 1, true),
                Literal::new(x, y + 2, true),
            ]);
            out.push(vec![
                Literal::new(x, y, false),
                Literal::new(x, y + 1, false),
                Literal::new(x, y + 2, false),
            ]);
        }
    }
    for y in 0..grid.size - 1 {
        for x in 0..grid.size - 3 {
            out.push(vec![
                Literal::new(x, y, true),
                Literal::new(x + 1, y, true),
                Literal::new(x + 2, y, true),
            ]);
            out.push(vec![
                Literal::new(x, y, false),
                Literal::new(x + 1, y, false),
                Literal::new(x + 2, y, false),
            ]);
        }
    }
}

/// Renvoie toutes les paires uniques (combinatoire) entre les éléments de `slice`, sous la forme
/// d'un itérateur (évaluation paresseuse).
fn pairs<T>(slice: &[T]) -> impl Iterator<Item = (&T, &T)> {
    std::iter::repeat(slice)
        .take(slice.len())
        .enumerate()
        .map(|(idx, slice)| &slice[idx..])
        .filter_map(|slice| slice.split_first())
        .map(|(first, seconds)| std::iter::repeat(first).zip(seconds))
        .flatten()
}

pub fn write_rule_3<W>(out: &mut CNFFile<W>, grid: &Grid) {
    #[derive(Clone, Copy, Eq, PartialEq, Hash)]
    enum ParamLiteral {
        A(usize, bool),
        B(usize, bool),
    }

    // « Une ligne/colonne A est différente d'une ligne/colonne B » en FND (paramétrique)
    let diff_a_b_dnf = (0..grid.size)
        .map(|z| {
            vec![
                [ParamLiteral::A(z, true), ParamLiteral::B(z, false)],
                [ParamLiteral::A(z, false), ParamLiteral::B(z, true)],
            ]
        })
        .flatten()
        .collect::<Box<[_]>>();

    // ... en FNC
    eprint!("| starting expansion...");
    let instant_exp = Instant::now();
    let diff_a_b_cnf = dnf_to_cnf(&diff_a_b_dnf.iter().map(|s| &s[..]).collect::<Vec<_>>()[..]);
    eprintln!(
        " DONE ({:?}) ({} clauses)",
        instant_exp.elapsed(),
        diff_a_b_cnf.len(),
    );

    // Liste des nombre de 0 à grid.size
    let indices = (0..grid.size).collect::<Box<[_]>>();

    // On s'occupe des listes et des colonnes dans la boucle for, puisque les paires sont les mêmes
    let instant_sub = Instant::now();
    let pair_count = (indices.len() * (indices.len() - 1)) / 2;
    for (idx, (a, b)) in pairs(&indices).enumerate() {
        let (a, b) = (*a, *b);

        eprint!("\r| substituting and writing... {}/{}", idx, pair_count);

        // Assignation de la forme paramétrique `diff_a_b_cnf` aux lignes
        let mut diff_cnf_l = diff_a_b_cnf.iter().map(|clause| {
            clause
                .iter()
                .map(|lit| match *lit {
                    ParamLiteral::A(x, neg) => Literal::new(x, a, neg),
                    ParamLiteral::B(x, neg) => Literal::new(x, b, neg),
                })
                .collect::<Vec<_>>()
        });

        // Assignation de la forme paramétrique `diff_a_b_cnf` aux colonnes
        let mut diff_cnf_h = diff_a_b_cnf.iter().map(|clause| {
            clause
                .iter()
                .map(|lit| match *lit {
                    ParamLiteral::A(y, neg) => Literal::new(a, y, neg),
                    ParamLiteral::B(y, neg) => Literal::new(b, y, neg),
                })
                .collect::<Vec<_>>()
        });

        out.push_multiple(diff_cnf_l.chain(diff_cnf_h));
    }
    eprintln!(
        "\r| substituting and writing... DONE ({:?})",
        instant_sub.elapsed()
    );
}

pub fn write_all<W>(out: &mut CNFFile<W>, grid: &Grid) {
    write_rule_1(out, grid);
    write_rule_2(out, grid);

    eprintln!("[rule 3] starting rule");
    let instant_r3 = Instant::now();
    write_rule_3(out, grid);
    eprintln!("\\ DONE ({:?})", instant_r3.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn pairs_test() {
        const EXPECTED: &[[i32; 2]] = &[[1, 2], [2, 3], [3, 4], [4, 1], [1, 3], [2, 4]];

        let pairs: Vec<_> = pairs(&[1, 2, 3, 4])
            .map(|(e1, e2)| HashSet::<_>::from_iter(vec![*e1, *e2]))
            .collect();

        assert_eq!(EXPECTED.len(), pairs.len(), "{:?}", pairs);
        for expected_pair in EXPECTED {
            let expected_pair = &HashSet::<_>::from_iter(expected_pair.into_iter().copied());
            assert!(
                pairs.contains(expected_pair),
                "{:?} ∉ {:?}",
                expected_pair,
                pairs,
            );
        }
    }
}
