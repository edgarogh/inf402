use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket_contrib::serve::StaticFiles;

const PATH: &str = "ads/";

pub fn get_ads_routes() -> StaticFiles {
    StaticFiles::from(PATH)
}

pub struct AdProvider([(); 0]);

impl AdProvider {
    pub fn get(&self) -> Option<String> {
        let all: Vec<_> = std::fs::read_dir(PATH)
            .ok()?
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();

        use rand::seq::IteratorRandom;
        all.into_iter().choose(&mut rand::thread_rng())
    }
}

#[derive(Debug)]
pub enum Error {
    Unauthorized,
}

impl<'a, 'r> FromRequest<'a, 'r> for AdProvider {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        let cookie = cookies.get("ads");
        if let Some("yes") = cookie.map(|c| c.value()) {
            Outcome::Success(AdProvider([]))
        } else {
            Outcome::Failure((Status::Unauthorized, Error::Unauthorized))
        }
    }
}
