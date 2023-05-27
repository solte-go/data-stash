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
pub use web::api::API_KEY_HEADER;

use crate::domain::maintenance::Maintenance;
use crate::web::counter::HitCounter;
use data::AppDatabase;
use rocket::fs::FileServer;
use rocket::{Build, Rocket};
use web::render::Renderer;

pub struct RocketConfig {
    pub renderer: Renderer<'static>,
    pub database: AppDatabase,
    pub hit_counter: HitCounter,
    pub maintenance: Maintenance,
}

pub fn new_rocket(config: RocketConfig) -> Rocket<Build> {
    rocket::build()
        .manage::<AppDatabase>(config.database)
        .manage::<Renderer>(config.renderer)
        .manage::<HitCounter>(config.hit_counter)
        .manage::<Maintenance>(config.maintenance)
        .mount("/", web::http::routes())
        .mount("/api/clip", web::api::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", web::http::catcher::catchers())
        .register("/api/clip", web::api::catcher::catchers())
}

#[cfg(test)]
pub mod test {
    pub fn async_runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime")
    }
}
