extern crate actix_web;
#[macro_use]
extern crate json;

use actix_web::{
    web, 
    Error,
    HttpServer, 
    App, 
    Responder,
    HttpResponse,
};
use futures::{Future, Stream};
use json::JsonValue;

fn index() -> impl Responder {
    HttpResponse::Ok().body(
        "Hello from actix_web and Rust!"
    )
}

fn ping() -> impl Responder {
    HttpResponse::Ok().body(
        "pong"
    )
}

/// This handler manually load request payload and parse json-rust
fn validate_json(payload: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {
    payload.concat2().from_err().and_then(|body| {
        // body is loaded, now we can deserialize json-rust
        let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
        let json_body: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };
		println!("DEBUG: parsed json body(or error) is {}", json_body);
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(json_body.dump()))
    })
}

fn main() {
    HttpServer::new(|| { 
        App::new()
        .route("/", web::get().to(index))
        .route("/ping", web::get().to(ping))
        .route("/validate_json", web::post().to_async(validate_json))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .unwrap();
}
