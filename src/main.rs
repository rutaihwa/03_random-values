use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};
use log::{debug, info, trace};
use pretty_env_logger as logger;

fn main() {
    // Initialize logger
    logger::init();
    info!("Random microservice - v.0.1.0");

    trace!("Starting server");

    let addr = ([127, 0, 0, 1], 8080).into();

    debug!("Trying to bind server to address: {}", addr);
    let builder = Server::bind(&addr);

    trace!("Creating service handler");
    let server = builder.serve(|| {
        service_fn_ok(|req| {
            trace!("Incoming request is : {:?}", req);

            let random_byte = rand::random::<u8>();
            debug!("Generated value is: {}", random_byte);

            Response::new(Body::from(random_byte.to_string()))
        })
    });

    info!("Used address: {}", server.local_addr());
    let server = server.map_err(drop);

    debug!("Run");
    hyper::rt::run(server);
}
