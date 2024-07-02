use log::error;

mod errors;
mod headers;
mod media;
mod config;
mod requests;
mod server;

fn main() {
    pretty_env_logger::init_timed();
  
    let config = match config::parse() {
        Ok(c) => c,
        Err(e) => {
            error!("{e}");
            std::process::exit(2);
        }
    };

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", &config.logging.default_level);
    }

    match server::serve(config) {
        Ok(_) => (),
        Err(e) => {
            error!("{e}");
            std::process::exit(1);
        }
    }
}
