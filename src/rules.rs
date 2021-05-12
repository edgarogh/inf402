use crate::cnf::{CNFFile, Literal};
use crate::logic_utils::dnf_to_cnf;
use crate::Grid;
use std::time::Instant;

/// Credits: https://docs.python.org/3.9/library/itertools.html#itertools.combinations
fn combinations<T>(list: &[T], r: usize) -> Vec<Vec<&T>> {
    let n = list.len();
    if r > n {
        return Vec::new();
    }

    let mut ret: Vec<Vec<&T>> = Vec::new();

    ret.push(list[..r].into_iter().collect());
    let mut indices: Vec<_> = (0..r).collect();

    loop {
        let mut broken = None;

        for i in (0..r).rev() {
            if indices[i] != i + n - r {
                broken = Some(i);
                break;
            }
        }

        let i = if let Some(i) = broken {
            i
        } else {
            return ret;
        };

        indices[i] += 1;

        for j in (i + 1)..r {
            indices[j] = indices[j - 1] + 1
        }

        ret.push(indices.iter().map(|i| &list[*i]).collect())
    }
}

pub fn write_rule_1(out: &mut CNFFile, grid: &Grid) {
    for k in 0..grid.size {
        let row_or_line: Vec<_> = std::iter::repeat(k).enumerate().take(grid.size).collect();

        for combination in combinations(&row_or_line, grid.size / 2 + 1) {
            out.push(
                combination
                    .iter()
                    .map(|(k, l)| Literal::new(*k, *l, true))
                    .collect(),
            );
            out.push(
                combination
                    .iter()
                    .map(|(k, l)| Literal::new(*k, *l, false))
                    .collect(),
            );
            out.push(
                combination
                    .iter()
                    .map(|(k, l)| Literal::new(*l, *k, true))
                    .collect(),
            );
            out.push(
                combination
                    .iter()
                    .map(|(k, l)| Literal::new(*l, *k, false))
                    .collect(),
            );
        }
    }
}

pub fn write_rule_2(out: &mut CNFFile, grid: &Grid) {
    for x in 0..grid.size {
        for y in 0..grid.size - 2 {
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
    for y in 0..grid.size {
        for x in 0..grid.size - 2 {
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

pub fn write_rule_3(out: &mut CNFFile, grid: &Grid) {
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
        let diff_cnf_l = diff_a_b_cnf.iter().map(|clause| {
            clause
                .iter()
                .map(|lit| match *lit {
                    ParamLiteral::A(x, neg) => Literal::new(x, a, neg),
                    ParamLiteral::B(x, neg) => Literal::new(x, b, neg),
                })
                .collect::<Vec<_>>()
        });

        // Assignation de la forme paramétrique `diff_a_b_cnf` aux colonnes
        let diff_cnf_h = diff_a_b_cnf.iter().map(|clause| {
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

pub fn write_all(out: &mut CNFFile, grid: &Grid) {
    let run_rule = |out: &mut CNFFile, rule: fn(&mut CNFFile, &Grid), no: u8| {
        eprintln!("[rule {}] starting rule", no);
        let start = Instant::now();
        rule(out, grid);
        eprintln!("\\ DONE ({:?})", start.elapsed());
    };

    run_rule(out, write_rule_1, 1);
    run_rule(out, write_rule_2, 2);
    out.solve().unwrap();
    run_rule(out, write_rule_3, 3);
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

    #[test]
    fn combinations_test() {
        // eprintln!("{:#?}", combinations(4, 8));
        eprintln!("{:#?}", combinations(&['A', 'B', 'C', 'D'], 2));
    }
}
