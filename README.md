# http-server-rs

A simple HTTP server in Rust; very minimal.

## Why?

I spent a week writing an HTTP server for two main reasons:

- For fun
- To prove I could do it

This HTTP was without any helper libraries, except some error handling and logging ones of course.

## Building

Development build:<br/>
`cargo build`

Release build:<br/>
`cargo build --release`

## Roadmap

- [x] Send and receive requests
- [x] Minimal external library usage
- [x] Readable and fault-proof codebase
- [ ] Configuration system
- [ ] More efficient request handling
    - [ ] Multi-threading
    - [ ] Async
- [ ] HTTP 1.1 compliance?
