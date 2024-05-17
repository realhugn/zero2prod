use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    /*
        HttpServer::new does not take App as argument - it wants a closure that returns an App struct.
        This is to support actix-web’s runtime model: actix-web will spin up a worker process for each
        available core on your machine.
        Each worker runs its own copy of the application built by HttpServer calling the very same closure
        that HttpServer::new takes as argument ->  connection has to be cloneable => Wrap it in ARC which means
        each instance of the application will have a pointer to PgConnection instead of a raw copy of it.
     */
    
    // web::Data wraps our connection in an Atomic Reference Counted pointer
    let db_pool = web::Data::new(db_pool);
    
    let server = HttpServer::new(move || {
    App::new()
        .wrap(Logger::default())
        .route("/health_check", web::get().to(health_check))
        .route("/subscriptions", web::post().to(subscribe))
        // Register the connection as part of the application state
        .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)  
}
