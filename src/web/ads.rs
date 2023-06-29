use rand::seq::IteratorRandom;
use rocket::fairing::{AdHoc, Fairing};
use rocket::fs::FileServer;
use rocket::http::{Cookie, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

const PATH: &str = "ads/";

pub fn get_ads_routes() -> FileServer {
    FileServer::from(PATH)
}

struct Ads(Vec<String>);

pub struct AdProvider<'a>(Option<&'a str>);

impl<'a> AdProvider<'a> {
    pub fn fairing() -> impl Fairing {
        AdHoc::try_on_ignite("Ads Provider", |rocket| async {
            let mut all = vec![];

            let mut read_dir = tokio::fs::read_dir(PATH).await.unwrap();
            while let Some(entry) = read_dir.next_entry().await.unwrap() {
                let path = entry.file_name().to_string_lossy().to_string();
                all.push(path);
            }

            Ok(rocket.manage(Ads(all)))
        })
    }

    pub fn name(&self) -> Option<&str> {
        self.0
    }
}

#[derive(Debug)]
pub enum Error {
    Unauthorized,
    NoAdsAvailable,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdProvider<'r> {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        let cookie = cookies.get("ads");
        if let Some("yes") = cookie.map(Cookie::value) {
            let Some(ads) = request.rocket().state::<Ads>() else {
                return Outcome::Failure((Status::ServiceUnavailable, Error::NoAdsAvailable));
            };

            Outcome::Success(AdProvider(
                ads.0
                    .iter()
                    .choose(&mut rand::thread_rng())
                    .map(String::as_str),
            ))
        } else {
            Outcome::Failure((Status::Unauthorized, Error::Unauthorized))
        }
    }
}
