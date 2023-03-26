use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                // Wait for client to send us a message, but ignore the content for now
                let mut buf = [0; 512];
                loop {
                    let size = stream.read(&mut buf).unwrap();
                    if size == 0 {
                        println!("no more data to read");
                        break;
                    }
                    println!("{:?}", &buf[0..size]);
                    stream.write("+PONG\r\n".as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
