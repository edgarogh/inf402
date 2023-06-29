use crate::{Cell, Grid};
use reqwest::StatusCode;
use std::convert::TryFrom;

fn parse_grid(grid: &str) -> Result<Grid, String> {
    Grid::try_from(
        grid.chars()
            .filter_map(|char| match char {
                '.' => Some(Cell::Empty),
                '0' => Some(Cell::Filled(false)),
                '1' => Some(Cell::Filled(true)),
                _ => None,
            })
            .collect::<Vec<_>>(),
    )
    .map_err(|()| String::from("invalid grid size"))
}

pub async fn fetch(id: u32) -> Result<(Grid, Grid), String> {
    let url = format!("https://rcijeux.fr/drupal_game/20minutes/takuzu/grids/{id}.takj");

    let res = match reqwest::get(url).await {
        Ok(response) if response.status() == StatusCode::NOT_FOUND => {
            return Err(String::from("grid not found"))
        }
        Ok(response) => response.text().await.unwrap_or_default(),
        Err(err) => return Err(format!("{err}")),
    };

    let (mut start, mut sol) = (None, None);

    for line in res.lines() {
        if let Some(grid) = line.strip_prefix("grille:\"") {
            start = Some(parse_grid(grid));
        }

        if let Some(grid) = line.strip_prefix("solution:\"") {
            sol = Some(parse_grid(grid));
        }
    }

    Ok((start.unwrap()?, sol.unwrap()?))
}
