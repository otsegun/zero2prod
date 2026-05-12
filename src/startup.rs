use actix_web::{App, HttpServer, dev::Server, middleware::Logger, web};

use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run(listener: TcpListener, db_pool: sqlx::PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    // Wrap the pool using web::Data, which boils down to an Arc smart pointer
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
