use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};
use log::{debug, info, trace};
use pretty_env_logger as logger;
use std::env;

fn main() {
    // Argument parser and subcommands
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("run")
                .about("Run the server")
                .arg(
                    Arg::with_name("address")
                        .short("a")
                        .long("address")
                        .value_name("ADDRESS")
                        .help("Sets an address")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .help("Sets a custom config file")
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("key").about("Generates a secret key for cookies"))
        .get_matches();

    // Initialize logger
    logger::init();
    info!(" {} - {}", crate_name!(), crate_version!());

    trace!("Starting server");

    // Server address
    let addr = matches
        .value_of("address")
        .map(|s| s.to_owned())
        .or(env::var("ADDRESS").ok())
        .or(dotenv::var("ADDRESS").ok())
        .unwrap_or_else(|| "127.0.0.1:8080".into())
        .parse()
        .expect("Could'nt parse ADDRESS variable");

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
