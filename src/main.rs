use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuraton = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuraton.application_port);
    let listener = TcpListener::bind(address)?;
    println!("{:?}", &listener);
    run(listener)?.await
}
