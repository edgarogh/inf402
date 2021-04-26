use crate::cnf::{CNFFile, Literal};
use crate::Grid;

pub fn write_rule_1<W>(out: &mut CNFFile<W>, grid: &Grid) {
    // TODO
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
