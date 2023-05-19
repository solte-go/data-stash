pub mod data;
pub mod domain;
pub mod service;
pub mod web;

pub use data::DataError;
pub use domain::clip::field::Shortcode;
pub use domain::clip::ClipError;
pub use domain::time::Time;
pub use domain::Clip;
pub use service::ServiceError;

use data::AppDatabase;
use rocket::fs::FileServer;
use rocket::{Build, Rocket};
use web::render::Renderer;

pub struct RocketConfig {
    pub renderer: Renderer<'static>,
    pub database: AppDatabase,
}

pub fn new_rocket(config: RocketConfig) -> Rocket<Build> {
    rocket::build()
        .manage::<AppDatabase>(config.database)
        .manage::<Renderer>(config.renderer)
        .mount("/", web::http::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", web::http::catcher::catchers())
}
