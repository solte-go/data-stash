use clip_ctash::data::AppDatabase;
use clip_ctash::domain::maintenance::Maintenance;
use clip_ctash::web::counter::HitCounter;
use clip_ctash::web::render::Renderer;
use dotenv::dotenv;
use rocket::tokio;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "httpd")]
struct Opt {
    #[structopt(default_value = "sqlite:data.db")]
    connection_string: String,
    #[structopt(short, long, parse(from_os_str), default_value = "templates/")]
    template_directory: PathBuf,
}

fn main() {
    dotenv().ok();
    let opt = Opt::from_args();

    let rt = tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime");

    let handle = rt.handle().clone();
    let renderer = Renderer::new(opt.template_directory.clone());

    let database = rt.block_on(async move { AppDatabase::new(&opt.connection_string).await });

    let hit_counter = HitCounter::new(database.get_pool().clone(), handle.clone());
    let maintenance = Maintenance::spawn(database.get_pool().clone(), handle.clone());

    let config = clip_ctash::RocketConfig {
        renderer,
        database,
        hit_counter,
        maintenance,
    };

    rt.block_on(async move {
        clip_ctash::new_rocket(config)
            .launch()
            .await
            .expect("failed to lunch rocket server")
    });
}
