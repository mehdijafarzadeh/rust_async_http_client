// Import the `thiserror` crate's Error trait to handle custom errors in a simple and user-friendly way.
use thiserror::Error;

/// Custom error type for handling both I/O and HTTP errors.
///
/// This enum defines two possible error types:
/// - `Io` for handling `std::io::Error` during file or stream I/O operations.
/// - `HttpRequest` for handling `reqwest::Error` during HTTP requests.
#[derive(Error, Debug)]
pub enum MyError {
    /// I/O error variant with a detailed error message from `std::io::Error`.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// HTTP request error variant with a detailed error message from `reqwest::Error`.
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),
}

// The main function is asynchronous, thanks to `#[tokio::main]` and the `tokio` runtime.
// This function performs an HTTP GET request to the "https://httpbin.org/get" endpoint.
// If any error occurs, it will return an appropriate `MyError` variant.
#[tokio::main]
async fn main() -> Result<(), MyError> {
    // Perform a GET request using `reqwest::get` and wait for the response asynchronously.
    let res = reqwest::get("https://httpbin.org/get").await?;

    // Print the status of the HTTP response.
    println!("Status: {}", res.status());

    // Print the HTTP response headers in a pretty debug format.
    println!("Headers: {:#?}", res.headers());

    // Get the response body as a string and print it.
    let body = res.text().await?;
    println!("Body: {}", body);

    // Return `Ok(())` to indicate that the function executed successfully.
    Ok(())
}
