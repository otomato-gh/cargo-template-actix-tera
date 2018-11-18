extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate tera;

use std::collections::HashMap;

use actix_web::{
    error, http, middleware, server, App, Error, HttpResponse, Query, State,
};

struct AppState {
    template: tera::Tera, // <- store tera template in application state
}

fn index((state, _query): (State<AppState>, Query<HashMap<String, String>>),) -> Result<HttpResponse, Error> {
    let s = state
        .template
        .render("index.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn main() {
    ::std::env::set_var("RUST_LOG", "{{project-name}}=info");
    env_logger::init();
    let sys = actix::System::new("{{project-name}}");

    server::new(|| {
        let tera =
            compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));

        App::with_state(AppState{template: tera})
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(http::Method::GET).with(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
