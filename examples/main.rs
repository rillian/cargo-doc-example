use cargo_doc_example::check;
use env_logger::Env;
use log::info;

fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("Calling check()...");
    check();
    info!("Done.");
}
