use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub fn dnf_to_cnf<F: Copy + Eq + Hash>(dnf: &[&[F]]) -> Vec<HashSet<F>> {
    match dnf {
        [] => unimplemented!(),
        [con_clause] => con_clause
            .into_iter()
            .map(|lit| HashSet::from_iter(vec![lit].into_iter().copied()))
            .collect(),
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
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect()
        }
    }
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
