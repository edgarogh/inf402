use crate::cnf::{CNFFile, Literal};
use crate::Grid;

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
