use crate::errors::HttpErrors;
use crate::requests::handle_request;
use log::{error, info};
use std::{
    io::Read,
    net::TcpListener,
    str::from_utf8,
};

const PORT: u16 = 80;

pub fn serve() -> Result<(), HttpErrors> {
    let listener: TcpListener = TcpListener::bind(&format!("[::]:{PORT}")).map_err(|e| HttpErrors::TcpListenerBindFailure(PORT.to_string(), e.to_string()))?;
    info!("HTTP server online, open for connections on port: {PORT}");

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(e) => {
                error!("{e}");
                continue;
            }
        };

        let mut buf = [0; 1024];
        let bytes_read = match stream.read(&mut buf) {
            Ok(r) => r,
            Err(e) => {
                error!("{e}");
                continue;
            }
        };

        let request = match from_utf8(&buf[..bytes_read]) {
            Ok(r) => r,
            Err(e) => {
                error!("{e}");
                continue;
            }
        };

        if bytes_read == 0 {
            error!("Reading client data failed, fatal error, exiting...");
            return Ok(());
        }

        match handle_request(request, &mut stream) {
            Ok(_) => (),
            Err(e) => {
                error!("{e}");
                continue;
            }
        };
    }

    Ok(())
}
