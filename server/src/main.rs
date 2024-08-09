use server::Threadpool;
use std::io::{self, ErrorKind};
use std::time::Duration;
use std::{fs, thread};
use std::{
    io::{BufRead, BufReader, Result, Write},
    net::{TcpListener, TcpStream},
};

const ADDR: &str = "127.0.0.1:7878";
const STATUS_OK: &str = "HTTP/1.1 200 OK";

const GET_ROOT: &str = "GET / HTTP/1.1";
const SLEEP: &str = "GET /sleep HTTP/1.1";
const ERR: &str = "GET /testerr HTTP/1.1";

fn main() -> Result<()> {
    let listener = TcpListener::bind(ADDR)?;
    let pool = Threadpool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                pool.execute(|| match handle_connection(s) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error on handle connection: {}", e);
                    }
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut s: TcpStream) -> Result<()> {
    let buf = BufReader::new(&mut s);

    let maybe_req: Result<Vec<_>> = buf
        .lines()
        .take_while(|result| match result {
            Ok(line) => !line.is_empty(),
            Err(_) => true,
        })
        .collect();

    let req = match maybe_req {
        Ok(req) => req,
        Err(e) => return Err(e),
    };

    println!("Request was: {req:#?}");

    let req_line = &req[0];
    match req_line.as_str() {
        GET_ROOT => {
            return respond_root(s);
        }
        SLEEP => {
            thread::sleep(Duration::from_secs(5));
            return respond_root(s);
        }
        ERR => return Err(io::Error::new(ErrorKind::Other, "An I/O error occurred")),
        _ => {
            return respond_not_found(s);
        }
    }
}

fn respond_not_found(s: TcpStream) -> Result<()> {
    return respond_html(s, "404.html");
}

fn respond_root(s: TcpStream) -> Result<()> {
    return respond_html(s, "hello.html");
}

fn respond_html(mut s: TcpStream, file_path: &str) -> Result<()> {
    let cnt = fs::read_to_string(file_path)?;
    let len = cnt.len();
    let resp = format!("{STATUS_OK}\r\nContent-Length: {len}\r\n\r\n{cnt}");
    s.write_all(resp.as_bytes())?;

    return Ok(());
}
