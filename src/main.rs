use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("echo fn is running for a request to {}", req.uri().path());
    Ok(Response::new(Body::from(format!(
        "You requested path: {}",
        req.uri().path()
    ))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
