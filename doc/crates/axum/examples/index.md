# axum crate - example

[Runnable project](/projects/crates/axum/hello_world)

Example of using the axum crate to build a web service in Rust:

```rust
use axum::{Router, routing::get};
use std::net::SocketAddr;

async fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

In this example, we define a web service that receives HTTP GET requests, and responds with "Hello, World!".

We define an asynchronous function `hello`. It returns the static string "Hello, World!".

We define a router using the `Router::new()` function, and use the `route()` method to define a route that maps the root URL (`"/"`) to the hello_world handler function.

We create a SocketAddr object representing the address and port on which the web service will listen (`127.0.0.1:3000`), and print a message indicating that the service is listening on that address.

We use the `axum::Server` type to bind the address to the web service, and serve it using the `serve()` method.
