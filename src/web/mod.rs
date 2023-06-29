#[path = "ads.rs"]
mod ads_provider;
mod fetch_20_minutes;

use crate::cnf::CNFFile;
use crate::templates;
use crate::web::ads_provider::{get_ads_routes, AdProvider};
use crate::{Cell, Grid};
use fetch_20_minutes::fetch;
use once_cell::sync::Lazy;
use rand::distributions::uniform::SampleRange;
use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::http::{Cookie, CookieJar};
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::{Build, Rocket, State};
use std::convert::TryFrom;
use std::fmt::Write;
use std::sync::Mutex;
use std::time::Instant;

#[get("/?<id>")]
fn index(
    base_url: &State<String>,
    ads: Option<AdProvider>,
    id: Option<u32>,
) -> Result<RawHtml<Vec<u8>>, String> {
    let value = match id {
        Some(id) => {
            let grid = fetch(id)?.0;

            let mut out = String::with_capacity(grid.size.pow(2));
            writeln!(out, "{}", grid.size).unwrap();
            for y in 0..grid.size {
                for x in 0..grid.size {
                    out.push(grid.get(x, y).to_char());
                }
                out.push('\n');
            }
            Some(out)
        }
        None => None,
    };

    let sample = || (0u32..1884).sample_single(&mut rand::thread_rng());

    let ad = ads.and_then(|ads| ads.get());

    let mut out = Vec::new();
    templates::index_html(
        &mut out,
        base_url.inner(),
        ad.as_deref(),
        &[sample(), sample(), sample()],
        value.as_deref().unwrap_or_default(),
    )
    .unwrap();

    Ok(RawHtml(out))
}

static LOAD_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

#[derive(FromForm)]
struct SolutionForm {
    grid: String,
}

#[post("/solution", data = "<form>")]
fn solution(
    base_url: &State<String>,
    ads: Option<AdProvider>,
    form: Form<SolutionForm>,
) -> Result<RawHtml<Vec<u8>>, String> {
    let guard = LOAD_LOCK.lock().unwrap();

    let grid = {
        let grid_size: usize = match crate::grid_read::size(&form.grid) {
            None => return Err(String::from("Invalid size")),
            Some(size) if size.get() <= 8 => size.get(),
            Some(_size_too_big) => return Err(String::from("Grid too big, refusing to solve")),
        };

        let mut grid: Grid = Grid::new(grid_size);
        crate::grid_read::fill_grid_from_file(&mut grid, &form.grid);
        grid
    };

    let start = Instant::now();
    let mut output = CNFFile::new_varisat(&grid);
    crate::rules::write_all(&mut output, &grid);

    let model = match output.solve() {
        Ok(true) => {
            // eprintln!("\\ DONE ({:?})", instant_solving.elapsed());
            output.model().unwrap()
        }
        Ok(false) => {
            return Err(String::from("ERROR: unsat"));
        }
        Err(err) => {
            return Err(format!("ERROR: {err}"));
        }
    };

    std::mem::drop(guard); // Make sure the guard is kept until here

    match Grid::try_from(
        model
            .into_iter()
            .map(|lit| Cell::Filled(lit.is_positive()))
            .collect::<Vec<_>>(),
    ) {
        Ok(grid) => {
            let ad = ads.and_then(|ads| ads.get());

            let mut out = Vec::new();
            templates::solution_html(
                &mut out,
                base_url.inner(),
                ad.as_deref(),
                &form.grid,
                &format!("{}\n{}", grid.size, grid),
                &format!("{:?}", start.elapsed()),
            )
            .unwrap();
            Ok(RawHtml(out))
        }
        Err(()) => unreachable!("invalid model"),
    }
}

#[get("/ads")]
pub fn ads(base_url: &State<String>, cookies: &CookieJar) -> Redirect {
    cookies.add(Cookie::new("ads", "yes"));
    let base_url = base_url.inner().clone();
    Redirect::to(if base_url.is_empty() {
        "/".into()
    } else {
        base_url
    })
}

pub fn main_rocket(base_url: String) -> Rocket<Build> {
    rocket::build()
        .manage(base_url)
        .mount("/", routes![index, solution, ads])
        .mount("/static", FileServer::from("src/static"))
        .mount("/gnome", FileServer::from("gnome"))
        .mount("/ads", get_ads_routes())
}
