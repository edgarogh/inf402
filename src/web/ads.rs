use rocket::fs::FileServer;
use rocket::http::{Cookie, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

const PATH: &str = "ads/";

pub fn get_ads_routes() -> FileServer {
    FileServer::from(PATH)
}

pub struct AdProvider([(); 0]);

impl AdProvider {
    #[allow(clippy::unused_self)]
    pub fn get(&self) -> Option<String> {
        let all: Vec<_> = std::fs::read_dir(PATH)
            .ok()?
            .filter_map(Result::ok)
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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdProvider {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        let cookie = cookies.get("ads");
        if let Some("yes") = cookie.map(Cookie::value) {
            Outcome::Success(AdProvider([]))
        } else {
            Outcome::Failure((Status::Unauthorized, Error::Unauthorized))
        }
    }
}
