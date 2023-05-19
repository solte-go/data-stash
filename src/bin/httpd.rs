use clip_ctash::data::AppDatabase;
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

    rt.block_on(async move {
        let renderer = Renderer::new(opt.template_directory);
        let database = AppDatabase::new(&opt.connection_string).await;

        let config = clip_ctash::RocketConfig { renderer, database };
        clip_ctash::new_rocket(config)
            .launch()
            .await
            .expect("faild to lunch rocket sertver")
    });
}
