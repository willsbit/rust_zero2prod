use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    // Port 0 is special-cased at the OS level:
    // trying to bind port 0 will trigger an OS scan for an available port
    // which will then be bound to the application.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // Get the port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    // return the app address to the caller
    format!("http://127.0.0.1:{}", port)
}
