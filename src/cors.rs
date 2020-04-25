use rocket::fairing::{Fairing, Info, Kind};
use rocket::{http::Method, http::Status, Request, Response};

pub struct CorsFairing;

impl Fairing for CorsFairing {
    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        response.set_header(rocket::http::Header::new(
            "Access-Control-Allow-Origin",
            "*"
        ));

        // Respond to all `OPTIONS` requests with a `204` (no content) status
        if response.status() == Status::NotFound && request.method() == Method::Options {
            response.set_status(Status::NoContent);
        }
    }
}