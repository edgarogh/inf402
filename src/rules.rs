use crate::cnf::{CNFFile, Literal};
use crate::Grid;

fn dnf_to_cnf(dnf: &[&[Literal]]) -> Vec<Vec<Literal>> {
    match dnf {
        [] => unimplemented!(),
        [con_clause] => con_clause.map(|lit| vec![lit]).collect(),
        [start @ .., cnf1] => {
            let cnf2 = dnf_to_cnf(start);

            cnf2.into_iter()
                .map(|clause| {
                    // disjonction de cnf1 (FNC) et de clause (clause) -> FNC
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
