use log::error;

mod errors;
mod headers;
mod media;
mod requests;
mod server;

fn main() {
    pretty_env_logger::init_timed();
    
    match server::serve() {
        Ok(_) => (),
        Err(e) => {
            error!("{e}");
            std::process::exit(1);
        }
    }
}
