use std::io::{self, Read, Write};
use std::net::TcpListener;

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:9092")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut header = [0u8; 12];
                stream.read_exact(&mut header)?;

                let message_size_bytes = &header[0..4];
                let correlation_id_bytes = &header[8..12];

                let response = [message_size_bytes, correlation_id_bytes].concat();
                stream.write_all(&response)?;
            }
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }

    Ok(())
}
