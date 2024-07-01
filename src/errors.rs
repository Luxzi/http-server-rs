#![allow(unused)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpErrors {
    #[error("Failed to bind TcpListener to port `{0}`: `{1}`")]
    TcpListenerBindFailure(String, String),

    #[error("Failed to read TcpStream with error: `{0}`")]
    StreamReadFailure(String),

    #[error("Failed to write TcpStream with error: `{0}`")]
    StreamWriteFailure(String),

    #[error("Failed to flush TcpStream with error: `{0}`")]
    StreamFlushFailure(String),

    #[error("Unable to get peer address: `{0}`")]
    StreamPeerAddressUnknown(String),

    #[error("Failed to convert request body to UTF-8")]
    Utf8ConversionFailure,

    #[error("Read zero bytes from request body")]
    GeneralReadFailure,

    #[error("Unsupported extension: `{0}`")]
    UnsupportedExtension(String),

    #[error("Unsupported request type: `{0}`")]
    UnsupportedRequestType(String),

    #[error("Unsupported protocol")]
    UnsupportedProtocol,

    #[error("Invalid request: `{0}`")]
    InvalidRequest(String),

    #[error("Requester attempted to access path outside authorized root: `{0}`")]
    UnauthorizedPath(String),

    #[error("Server could not locate resource at `{0}`")]
    ResourceNotFound(String),

    #[error("Failed to read file: `{0}`")]
    FileReadFailure(String),
}
