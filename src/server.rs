use crate::errors::HttpErrors;
use crate::config::Config;
use crate::requests::handle_request;
use log::{error, info};
use std::{
    io::Read,
    net::TcpListener,
    str::from_utf8,
};

pub fn serve(config: Config) -> Result<(), HttpErrors> {
    let listener: TcpListener = TcpListener::bind(&format!("{}:{}", config.server.address, config.server.port)).map_err(|e| HttpErrors::TcpListenerBindFailure(config.server.port.clone(), e.to_string()))?;
    info!("HTTP server online, open for connections on port: {}", config.server.port);

    for stream in listener.incoming() {
        let mut stream = match stream.map_err(|e| HttpErrors::StreamAcceptFailure(e.to_string())) {
            Ok(s) => s,
            Err(e) => {
                error!("{e}");
                continue;
            }
        };

        let mut buf = [0; 1024];
        let bytes_read = match stream.read(&mut buf).map_err(|e| HttpErrors::StreamReadFailure(e.to_string())) {
            Ok(r) => r,
            Err(e) => {
                error!("{e}");
                continue;
            }
        };

        let request = match from_utf8(&buf[..bytes_read]).map_err(|_| HttpErrors::Utf8ConversionFailure) {
            Ok(r) => r,
            Err(e) => {
                error!("{e}");
                continue;
            }
        };

        if bytes_read == 0 {
            return Err(HttpErrors::StreamReadFailure(String::from("Received 0 bytes, unknown")));
        }

        match handle_request(request, &mut stream, config) {
            Ok(_) => (),
            Err(e) => {
                error!("{e}");
                continue;
            }
        };
    }

    Ok(())
}
