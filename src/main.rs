use std::io;
use std::net::TcpListener;

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:9092")?;

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {}
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }

    Ok(())
}
