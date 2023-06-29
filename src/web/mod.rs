#[path = "ads.rs"]
mod ads_provider;
mod fetch_20_minutes;

use crate::cnf::CNFFile;
use crate::templates;
use crate::web::ads_provider::{get_ads_routes, AdProvider};
use crate::{Cell, Grid};
use fetch_20_minutes::fetch;
use rand::distributions::uniform::SampleRange;
use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::http::{Cookie, CookieJar};
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::{Build, Rocket, State};
use std::convert::TryFrom;
use std::fmt::Write;
use std::time::Instant;
use tokio::sync::Mutex;

#[get("/?<id>")]
async fn index(
    base_url: &State<String>,
    ads: Option<AdProvider<'_>>,
    id: Option<u32>,
) -> Result<RawHtml<Vec<u8>>, String> {
    let value = match id {
        Some(id) => {
            let grid = fetch(id).await?.0;

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

    let sample = || (2001u32..=2669).sample_single(&mut rand::thread_rng());

    let ad = ads.as_ref().and_then(AdProvider::name);

    let mut out = Vec::new();
    templates::index_html(
        &mut out,
        base_url.inner(),
        ad,
        &[sample(), sample(), sample()],
        value.as_deref().unwrap_or_default(),
    )
    .unwrap();

    Ok(RawHtml(out))
}

#[derive(FromForm)]
struct SolutionForm {
    grid: String,
}

#[post("/solution", data = "<form>")]
async fn solution(
    base_url: &State<String>,
    ads: Option<AdProvider<'_>>,
    form: Form<SolutionForm>,
    load_lock: &State<Mutex<()>>,
) -> Result<RawHtml<Vec<u8>>, String> {
    let guard = load_lock.lock().await;

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

    // TODO: this isn't cancellable
    let solution = tokio::task::spawn_blocking(move || {
        let mut output = CNFFile::new_varisat(&grid);
        crate::rules::write_all(&mut output, &grid);
        output.solve().map(|success| {
            if success {
                Some(output.model().unwrap())
            } else {
                None
            }
        })
    })
    .await
    .unwrap();

    println!("solution");

    let model = match solution {
        Ok(Some(model)) => model,
        Ok(None) => {
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
            let ad = ads.as_ref().and_then(AdProvider::name);

            let mut out = Vec::new();
            templates::solution_html(
                &mut out,
                base_url.inner(),
                ad,
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
        .attach(AdProvider::fairing())
        .manage(base_url)
        .manage(Mutex::new(()))
        .mount("/", routes![index, solution, ads])
        .mount("/static", FileServer::from("src/static"))
        .mount("/gnome", FileServer::from("gnome"))
        .mount("/ads", get_ads_routes())
}
