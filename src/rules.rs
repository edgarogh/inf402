use crate::cnf::{CNFFile, Literal};
use crate::logic_utils;
use crate::Grid;

pub fn write_rule_1<W>(out: &mut CNFFile<W>, grid: &Grid) {
    // TODO
}

pub fn write_rule_2<W>(out: &mut CNFFile<W>, grid: &Grid) {
    // TODO
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
