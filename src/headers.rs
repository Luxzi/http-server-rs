#![allow(unused)]

/// Supported HTTP specification. This is hard-coded because this is a mini-http server.
const HTTP_SPEC: f32 = 1.1;

/// Holds an HTTP status code with its corresponding phrase (i.e. 200 OK)
pub struct HttpStatusCode {
    pub code: u16,
    pub phrase: &'static str,
}

/// Holds the [`HTTP_SPEC`] constant for symantic reasons.
struct HttpSpec {
    version: f32,
}

/// Holds the data used to build the [`HttpStatusHeader`] (e.g. HTTP/1.1 200 OK).
pub struct HttpStatusHeader {
    spec: HttpSpec,
    code: HttpStatusCode,
}

/// Holds the content type information that is used in the [`HttpContentTypeHeader`].
/// e.g. text/plain
pub struct HttpContentType {
    pub first: &'static str,
    pub second: &'static str,
}

/// Holds the content type provided by [`HttpContentType`].
pub struct HttpContentTypeHeader {
    content_type: HttpContentType,
}

/// Holds the content to be to the client
#[derive(Default)]
pub struct HttpContent {
    pub content: Vec<u8>,
}

// TODO: Implement chunk sending
/// Holds a chunk of [`HttpContent`]. This is used to send larger files, such as images.
pub struct HttpContentChunk {
    content: HttpContent,
    current_chunk: u32,
    total_chunks: u32,
}

#[derive(Default)]
pub struct HttpResponse {
    pub headers: String,
    pub content: HttpContent,
}

/// Holds the size of the content of the response, in bytes.
pub struct HttpContentLengthHeader {
    length: usize,
}

/// Vector of HTTP headers used to build the response
pub struct HeaderVec {
    vec: Vec<Box<dyn StringifyHttpHeader>>,
}

/// [`HttpHeader`] supertrait.
pub trait HttpHeader<T> {
    fn new() -> Self;
    fn new_from(value: T) -> Self;
}

/// Allow an [`HttpHeader`] data type to be converted into its string form.
pub trait StringifyHttpHeader {
    fn to_string(&self) -> String;
}

/// Builds desired headers into a [`HeaderVec`]
#[macro_export]
macro_rules! headers {
    (
        $(
            $(#[docs:meta])*
            $header:expr;
        )+
    ) => {
        $crate::headers::HeaderVec::new()$(.add_header($header))+
    };
}

/// Builds the final response sent to the client.
#[macro_export]
macro_rules! response {
    (
        $(
            $headers:expr
        ),*; $content:expr
    ) => {
        $crate::headers::HttpResponseBuilder::new_from(
            headers!(
                $(
                    $headers;
                )*
            ),
            $content
        ).build()
    };
}

/// Creates constants for the provided HTTP status codes.
macro_rules! status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($code:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        $(
            $(#[$docs])*
            pub const $konst: HttpStatusCode = HttpStatusCode { code: $code, phrase: $phrase };
        )+

        /// Returns the phrase that corresponds to the provided code.
        fn code_to_msg(code: u16) -> Option<&'static str> {
            match code {
                $(
                $code => Some($phrase),
                )+
                _ => None,
            }
        }

        /// Returns an [`HttpStatusCode`] struct of the provided code.
        fn code_to_const(code: u16) -> Option<HttpStatusCode> {
            match code {
                $(
                $code => Some($konst),
                )+
                _ => None,
            }
        }
    };
}

/// Shorthand macro to create the final status code string
#[macro_export]
macro_rules! status_code_string {
    (
        $code:expr, $phrase:expr
    ) => {
        format!("{} {}", $code, $phrase)
    };
}

status_codes!(
    /// 200 OK
    (200, OK, "OK");

    /// 404 Not found
    (404, NOT_FOUND, "Not found");

    /// 401 Unauthorized
    (401, UNAUTHORIZED, "Unauthorized");

    /// 400 Bad Request
    (400, BAD_REQUEST, "Bad Request");

    /// 501 Not implemented
    (501, NOT_IMPLEMENTED, "Not Implemented");
);

impl HttpStatusCode {
    /// Creates a new [`HttpStatusCode`] with a default of OK (i.e. 200).
    /// This is not the preferred method of creating a new [`HttpStatusCode`]
    /// please use the [`new_from`](fn@Self::new_from) function instead.
    pub fn new() -> Self {
        OK
    }

    /// Creates a new [`HttpStatusCode`] with the specified status code.
    pub fn new_from(code: u16) -> Self {
        code_to_const(code).unwrap()
    }
}

impl HeaderVec {
    /// Creates an empty [`HeaderVec`].
    pub fn new() -> Self {
        HeaderVec { vec: vec![] }
    }

    /// Pushes a provided header into the [`HeaderVec`].
    pub fn add_header(mut self, header: impl StringifyHttpHeader + 'static) -> Self {
        self.vec.push(Box::new(header));

        self
    }

    /// Builds the [`HeaderVec`] into its [`String`] form
    pub fn build(self) -> String {
        let mut headers = self
            .vec
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("\r\n");
        headers.push_str("\r\n\r\n");
        headers
    }
}

impl Default for HttpStatusCode {
    /// Default implementation for [`HttpStatusCode`]. Uses `200 OK`.
    fn default() -> Self {
        HttpStatusCode::new_from(200)
    }
}

impl<T> From<T> for HttpContent
where
    T: AsRef<[u8]>,
{
    /// Takes any type and wraps it into an [`HttpContent`] struct.
    fn from(content: T) -> Self {
        HttpContent {
            content: content.as_ref().to_vec(),
        }
    }
}

impl HttpHeader<u16> for HttpStatusHeader {
    /// Creates a new [`HttpStatusHeader`] with the default status code of 200.
    /// This is not the preferred method of creating a new [`HttpStatusHeader`]
    /// please use the [`new_from`](fn@Self::new_from) function instead.
    fn new() -> Self {
        HttpStatusHeader {
            spec: HttpSpec { version: HTTP_SPEC },
            code: HttpStatusCode::new_from(200),
        }
    }

    /// Creates a new [`HttpStatusHeader`] with the specified status code.
    fn new_from(code: u16) -> Self {
        HttpStatusHeader {
            spec: HttpSpec { version: HTTP_SPEC },
            code: HttpStatusCode::new_from(code),
        }
    }
}

impl HttpHeader<HttpContentType> for HttpContentTypeHeader {
    /// Creates a new [`HttpContentTypeHeader`] with a default content type of text/plain.
    /// This is not the preferred method of creating a new [`HttpContentTypeHeader`]
    /// please use the [`new_from`](fn@Self::new_from) function instead.
    fn new() -> Self {
        HttpContentTypeHeader {
            content_type: HttpContentType {
                first: "text",
                second: "plain",
            },
        }
    }

    /// Creates a new [`HttpContentTypeHeader`] with the specified content type.
    fn new_from(content_type: HttpContentType) -> Self {
        HttpContentTypeHeader { content_type }
    }
}

impl HttpHeader<usize> for HttpContentLengthHeader {
    /// Creates a new [`HttpContentLengthHeader`] with a default length of 0.
    /// This is not the preferred method of creating a new [`HttpContentLengthHeader`]
    /// please use the [`new_from`](fn@Self::new_from) function instead.
    fn new() -> Self {
        HttpContentLengthHeader { length: 0 }
    }

    /// Creates a new [`HttpContentLengthHeader`] with the specified length.
    fn new_from(length: usize) -> Self {
        HttpContentLengthHeader { length }
    }
}

impl StringifyHttpHeader for HttpStatusHeader {
    /// `.to_string()` implementation for [`HttpStatusHeader`].
    fn to_string(&self) -> String {
        format!(
            "HTTP/{} {} {}",
            self.spec.version, self.code.code, self.code.phrase
        )
    }
}

impl StringifyHttpHeader for HttpContentTypeHeader {
    /// `.to_string()` implementation for [`HttpContentTypeHeader`].
    fn to_string(&self) -> String {
        format!(
            "Content-Type: {}/{}",
            self.content_type.first, self.content_type.second
        )
    }
}

impl StringifyHttpHeader for HttpContentLengthHeader {
    /// `.to_string()` implementation for [`HttpContentLengthHeader`].
    fn to_string(&self) -> String {
        format!("Content-Length: {}", self.length)
    }
}
