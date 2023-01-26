use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::{env, io, thread};

fn main() {
    // Bind the server's socketS
    // get port from env

    let port = env::var("PORT").unwrap_or("8888".to_string());
    let mut bind_addr = "0.0.0.0:".to_owned();
    bind_addr.push_str(&port);

    let remote_smtp_server = env::var("SMTP_SERVER").unwrap_or("send.one.com:587".to_string());

    let listener = TcpListener::bind(bind_addr).unwrap();

    loop {
        let (smtp_client, _) = listener.accept().unwrap();

        let smtp_server = TcpStream::connect(&remote_smtp_server).unwrap();

        // create arc for the sockets

        let smtp_client_arc = Arc::new(smtp_client);
        let smtp_server_arc = Arc::new(smtp_server);

        let (mut smtp_client_read, mut smtp_client_write) = (
            smtp_client_arc.try_clone().unwrap(),
            smtp_client_arc.try_clone().unwrap(),
        );
        let (mut smtp_server_read, mut smtp_server_write) = (
            smtp_server_arc.try_clone().unwrap(),
            smtp_server_arc.try_clone().unwrap(),
        );

        let client_to_server_thread = thread::spawn(move || {
            io::copy(&mut smtp_client_read, &mut smtp_server_write).unwrap_or(1)
        });
        let server_to_client_thread = thread::spawn(move || {
            io::copy(&mut smtp_server_read, &mut smtp_client_write).unwrap_or(1)
        });

        server_to_client_thread.join().unwrap();

        // close the connection to the client
        smtp_client_arc.shutdown(std::net::Shutdown::Both).unwrap();

        client_to_server_thread.join().unwrap();
    }
}
