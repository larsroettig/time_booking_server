extern crate actix_rt;
extern crate actix_web;
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate juniper;

use std::{env, io};
use actix_web::{middleware, App, HttpServer};

use graphql_time::db::get_pool;
use graphql_time::endpoints::graphql_endpoints;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    logging_setup();

    // Instantiate a new connection pool
    let pool = get_pool();

    let url = option_env!("GRAPHQL_URL");

    // Start up the server, passing in (a) the connection pool
    // to make it available to all endpoints and (b) the configuration
    // function that adds the /graphql logic.
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(graphql_endpoints)
    })
        .bind(url.unwrap_or("127.0.0.1:8080").to_string())?
        .run()
        .await
}

// TODO: more fine-grained logging setup
fn logging_setup() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}