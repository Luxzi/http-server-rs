use log::info;
use std::io::Read;
use std::{io::Write, net::TcpStream};
use fs_err as fs;

use crate::errors::HttpErrors;
use crate::{headers, status_code_string};
use crate::headers::{
    HttpContent, HttpContentLengthHeader, HttpContentTypeHeader, HttpHeader, HttpResponse, HttpStatusHeader, BAD_REQUEST, NOT_FOUND, NOT_IMPLEMENTED, UNAUTHORIZED
};

use crate::media::ext_to_type;

pub fn handle_request(request: &str, stream: &mut TcpStream) -> Result<(), HttpErrors> {
    let request_sections = request.split_whitespace().collect::<Vec<&str>>();
    let request_type = request_sections[0];
    let request_url = to_local_path(request_sections[1]);
    let request_ext = request_url.split(".").last();
    let ext = if request_ext.is_some() {
        String::from(request_ext.unwrap())
    } else {
        String::from("txt")
    };

    info!("Received {request_type} request from {:#?}", stream.peer_addr().map_err(|e| HttpErrors::StreamPeerAddressUnknown(e.to_string()))?);
    let response = match request_type {
        "GET" => get(request_url, ext),
        "POST" => post(request_url, ext),
        "PATCH" => patch(request_url, ext),
        "PUT" => put(request_url, ext),
        "DELETE" => delete(request_url, ext),
        _ => return Err(HttpErrors::UnsupportedRequestType(request_type.to_string()).into()),
    }?;

    stream
        .write(response.headers.as_bytes())
        .map_err(|e| HttpErrors::StreamWriteFailure(e.to_string()))?;
    stream
        .write(&response.content.content)
        .map_err(|e| HttpErrors::StreamWriteFailure(e.to_string()))?;
    stream
        .flush()
        .map_err(|e| HttpErrors::StreamFlushFailure(e.to_string()))?;
    info!("Sent response to {}", stream.peer_addr().map_err(|e| HttpErrors::StreamPeerAddressUnknown(e.to_string()))?);

    Ok(())
}

fn get(request_url: String, ext: String) -> Result<HttpResponse, HttpErrors> {
    let mut status_code = 200;
    let mut ext = ext;
    let mut content_buffer = vec![];
    let file_handle: Option<fs::File> = match get_file(&request_url) {
        Ok(f) => Some(f),
        Err(HttpErrors::ResourceNotFound(_)) => {
            status_code = 404;
            None
        },
        Err(HttpErrors::UnauthorizedPath(_)) => {
            status_code = 504;
            None
        },
        Err(e) => return Err(e),
    };

    let mut content: HttpContent;
    if status_code == 200 {
        file_handle.unwrap().read_to_end(&mut content_buffer).map_err(|e| HttpErrors::FileReadFailure(e.to_string()))?;

        content = HttpContent::from(content_buffer);
    } else {
        match status_code {
            404 => {
                content = HttpContent::from(status_code_string!(NOT_FOUND.code, NOT_FOUND.phrase));
            },

            401 => {
                content = HttpContent::from(status_code_string!(UNAUTHORIZED.code, UNAUTHORIZED.phrase));
            },

            _ => panic!(),
        }
    }

    if ext_to_type(&ext).is_none() {
        ext = String::from("txt");
        status_code = BAD_REQUEST.code;
        content = HttpContent::from(status_code_string!(BAD_REQUEST.code, BAD_REQUEST.phrase));
    }

    Ok(HttpResponse {
        headers: headers!(
            HttpStatusHeader::new_from(status_code);
            HttpContentTypeHeader::new_from(ext_to_type(&ext).unwrap().content_type);
            HttpContentLengthHeader::new_from(content.content.len());
        )
        .build(),
        content,
    })
}

fn post(_request_url: String, _ext: String) -> Result<HttpResponse, HttpErrors> {
    not_impl()
}

fn patch(_request_url: String, _ext: String) -> Result<HttpResponse, HttpErrors> {
    not_impl()
}

fn put(_request_url: String, _ext: String) -> Result<HttpResponse, HttpErrors> {
    not_impl()
}
fn delete(_request_url: String, _ext: String) -> Result<HttpResponse, HttpErrors> {
    not_impl()
}

fn to_local_path(path: &str) -> String {
    format!(".{path}")
}

fn get_file(path: &str) -> Result<fs::File, HttpErrors> {
    if path.find("..").is_some() {
        return Err(HttpErrors::UnauthorizedPath(path.to_string()).into());
    }

    if !std::path::PathBuf::from(path).exists() {
        return Err(HttpErrors::ResourceNotFound(path.to_string()).into());
    }

    let file = fs::File::open(path).map_err(|e| HttpErrors::FileReadFailure(e.to_string()))?;

    Ok(file)
}

fn not_impl() -> Result<HttpResponse, HttpErrors> {
    let content = HttpContent { content: status_code_string!(NOT_IMPLEMENTED.code, NOT_IMPLEMENTED.phrase).as_bytes().to_vec() };
    Ok(HttpResponse {
        headers: headers!(
            HttpStatusHeader::new_from(501);
            HttpContentTypeHeader::new_from(ext_to_type("txt").unwrap().content_type);
            HttpContentLengthHeader::new_from(content.content.len());
        )
        .build(),
        content,
    })
}
