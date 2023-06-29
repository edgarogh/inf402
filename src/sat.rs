use super::*;
use std::io::BufRead;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// Le SAT-Solveur n'a pas su trouver de modèle
    Unsatisfiable,

    /// Le fichier ne contient aucune ligne représentant un modèle
    UndefinedModel,

    /// Le format d'une ligne du fichier n'est pas reconnu
    InvalidLine(String),

    /// Le format du littéral n'est pas reconnu
    InvalidLiteral(String),

    /// Le nombre de variables du modèle n'a pas de racine entière: impossible de déterminer la
    /// taille de la grille
    InvalidModel,
}

fn create_grid(model: &str) -> Result<Grid, Error> {
    let model = model
        .strip_suffix(" 0")
        .ok_or(Error::InvalidLine(model.into()))?;

    let cells: Vec<_> = model
        .split_ascii_whitespace()
        .enumerate()
        .map(|(idx, lit)| {
            let (atom, is_one) = if let Some(atom) = lit.strip_prefix('-') {
                (atom, false)
            } else {
                (lit, true)
            };

            if (idx + 1).to_string() == atom {
                Ok(Cell::Filled(is_one))
            } else {
                Err(Error::InvalidLiteral(lit.into()))
            }
        })
        .collect::<Result<_, _>>()?;

    Grid::try_from(cells).map_err(|()| Error::InvalidModel)
}

/// Lis un fichier de résultats du SAT-Solveur et retourne la grille correspondante. Supporte les
/// formats:
///   - MiniSAT
///   - Varisat
///   - <http://www.satcompetition.org/2004/format-solvers2004.html>
pub fn read_sat_file(reader: impl BufRead) -> Result<Grid, Error> {
    for line in reader.lines() {
        let line = line.unwrap();

        match line.trim() {
            "SAT" | "s SATISFIABLE" | "" => continue,
            line if line.starts_with("c ") => continue,

            "UNSAT" | "s UNSATISFIABLE" => return Err(Error::Unsatisfiable),

            model
                if model
                    .chars()
                    .next()
                    .filter(|c| *c == '-' || c.is_numeric())
                    .is_some() =>
            {
                return create_grid(model)
            }

            // line if let Some(model) = line.strip_suffix("v ") => return create_grid(model), [PAS ENCORE STABLE]
            line if line.starts_with("v ") => return create_grid(&line[2..]),

            line => return Err(Error::InvalidLine(line.into())),
        }
    }

    Err(Error::UndefinedModel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsat() {
        const FILE: &str = "c File created by MyIncredibleSAT\n\ns UNSATISFIABLE\n";

        assert_eq!(
            read_sat_file(FILE.as_bytes()).unwrap_err(),
            Error::Unsatisfiable,
        );
    }

    #[test]
    fn sat_no_model() {
        const FILE: &str = "s SATISFIABLE\n";

        assert_eq!(
            read_sat_file(FILE.as_bytes()).unwrap_err(),
            Error::UndefinedModel,
        );
    }

    #[test]
    fn sat_minisat() {
        const FILE: &str = "SAT\n-1 2 -3 4 0\n";

        let grid = read_sat_file(FILE.as_bytes()).unwrap();

        assert_eq!(
            grid.inner,
            vec![
                Cell::Filled(false),
                Cell::Filled(true),
                Cell::Filled(false),
                Cell::Filled(true),
            ]
        );
    }

    #[test]
    fn sat_varisat() {
        const FILE: &str = "s SATISFIABLE\nv -1 2 -3 4 0\n";

        let grid = read_sat_file(FILE.as_bytes()).unwrap();

        assert_eq!(
            grid.inner,
            vec![
                Cell::Filled(false),
                Cell::Filled(true),
                Cell::Filled(false),
                Cell::Filled(true),
            ]
        );
    }
}
