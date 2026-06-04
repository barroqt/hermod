use std::io::{self, Read, Write};
use std::net::TcpListener;

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:9092")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("Connection handler failed: {e}");
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: impl Read + Write) -> Result<(), io::Error> {
    loop {
        let mut size_buf = [0u8; 4];
        match stream.read_exact(&mut size_buf) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }

        let request_size = i32::from_be_bytes(size_buf) as usize;
        let mut request = vec![0u8; request_size];
        stream.read_exact(&mut request)?;

        let correlation_id = &request[4..8];
        let request_api_version = i16::from_be_bytes([request[2], request[3]]);

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

    Ok(())
}
