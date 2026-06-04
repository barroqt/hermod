use std::io::{self, Read, Write};
use std::net::TcpListener;

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:9092")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut header = [0u8; 12];
                stream.read_exact(&mut header)?;

                let correlation_id = &header[8..12];
                let request_api_version = i16::from_be_bytes([header[6], header[7]]);

                let error_code: i16 = if request_api_version > 4 { 35 } else { 0 };

                let mut body = Vec::new();
                body.extend_from_slice(&error_code.to_be_bytes());

                if error_code == 0 {
                    body.push(0x02);
                    body.extend_from_slice(&18i16.to_be_bytes());
                    body.extend_from_slice(&0i16.to_be_bytes());
                    body.extend_from_slice(&4i16.to_be_bytes());
                    body.push(0x00);
                } else {
                    body.push(0x01);
                }

                body.extend_from_slice(&0i32.to_be_bytes());
                body.push(0x00);

                let message_size = 4 + body.len() as i32;
                let mut response = Vec::new();
                response.extend_from_slice(&message_size.to_be_bytes());
                response.extend_from_slice(correlation_id);
                response.extend_from_slice(&body);

                stream.write_all(&response)?;
            }
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }

    Ok(())
}
