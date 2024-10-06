# Async HTTP Client with Error Handling in Rust

This is a simple Rust project that demonstrates how to perform an asynchronous HTTP GET request using the `reqwest` crate. The project also implements error handling with `thiserror` to manage potential I/O and HTTP-related errors.

## Features

- Asynchronous HTTP requests using `reqwest`.
- Error handling using the `thiserror` crate.
- Prints the HTTP response's status, headers, and body.

## Project Structure

The main components of this project include:
- **Custom Error Handling**: The `MyError` enum handles I/O errors and HTTP request errors gracefully.
- **Asynchronous Execution**: The `#[tokio::main]` macro enables asynchronous execution in the `main` function.

## Dependencies

The following dependencies are used in this project:

- [`reqwest`](https://crates.io/crates/reqwest) for making HTTP requests.
- [`thiserror`](https://crates.io/crates/thiserror) for error handling.
- [`tokio`](https://crates.io/crates/tokio) as the asynchronous runtime.

Here are the versions of the dependencies:

```toml
[dependencies]
reqwest = { version = "0.12.8", features = ["blocking"] }
thiserror = "1.0.64"
tokio = { version = "1.40.0", features = ["full"] }
