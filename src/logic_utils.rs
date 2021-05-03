use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use std::rc::Rc;

fn dnf_to_cnf_rec<'a, F: 'a + Copy + Eq + Hash>(
    mut dnf: impl Iterator<Item = &'a [F]>,
) -> Vec<HashSet<F>> {
    let con_clause = dnf.next().expect("unimplemented: empty dnf");

    let con_clause_as_cnf: Vec<_> = con_clause
        .into_iter()
        .map(|lit| HashSet::from_iter(std::iter::once(lit).copied()))
        .collect();

    dnf.fold(con_clause_as_cnf, |acc, con_clause| {
        acc.into_iter()
            .map(|clause| {
                let clause = Rc::new(clause);

                con_clause
                    .into_iter()
                    .map(|lit| HashSet::from_iter(std::iter::once(lit).copied()))
                    .zip(std::iter::repeat(clause))
                    .map(|(mut new_clause, clause)| {
                        new_clause.extend(clause.iter().cloned());
                        new_clause
                    })
            })
            .flatten()
            .collect()
    })
}

pub fn dnf_to_cnf<F: Copy + Eq + Hash>(dnf: &[&[F]]) -> Vec<HashSet<F>> {
    dnf_to_cnf_rec(dnf.into_iter().copied())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dnf_to_cnf_simple() {
        const EXPECTED_CNF: &[&[char]] = &[
            &['c', 'a'],
            &['d', 'a'],
            &['c', 'b'],
            &['b', 'd'],
            &['c'],
            &['c', 'd'],
        ];

        let cnf = dnf_to_cnf(&[&['a', 'b', 'c'], &['c', 'd']]);

        assert_eq!(cnf.len(), EXPECTED_CNF.len());
        for clause in EXPECTED_CNF {
            assert!(cnf.contains(&HashSet::from_iter(clause.into_iter().copied())))
        }
    }

    #[test]
    fn dnf_to_cnf_less_simple() {
        const EXPECTED_CNF: &[&[char]] = &[
            &['c', 'a', 'e'],
            &['d', 'a', 'e'],
            &['c', 'b', 'e'],
            &['b', 'd', 'e'],
            &['c', 'e'],
            &['c', 'd', 'e'],
        ];

        let cnf = dnf_to_cnf(&[&['a', 'b', 'c'], &['c', 'd'], &['e']]);

        assert_eq!(cnf.len(), EXPECTED_CNF.len());
        for clause in EXPECTED_CNF {
            assert!(cnf.contains(&HashSet::from_iter(clause.into_iter().copied())))
        }
    }

    #[test]
    #[should_panic]
    fn empty() {
        dnf_to_cnf::<()>(&[]);
    }
}
