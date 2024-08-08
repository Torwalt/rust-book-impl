use std::io::{BufRead, BufReader, Result};
use std::net::{TcpListener, TcpStream};

const ADDR: &str = "127.0.0.1:7878";

fn main() -> Result<()> {
    let listener = TcpListener::bind(ADDR)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut s: TcpStream) {
    let buf = BufReader::new(&mut s);
    let req: Vec<_> = buf
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request was: {req:#?}")
}
