use std::net::TcpListener;
use std::io::Read;
use crate::http::{Request, Response, StatusCode};

pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr: addr
        }
    }

    pub fn run(&mut self) {
        println!("Listening to {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        
        'request_loop: loop {
            match listener.accept() {
                // ignore the address value by using "_" instead
                Ok((mut stream, _)) => {
                    let mut buf: [u8; 1024] = [0; 1024];
                    let response: Response;

                    match stream.read(&mut buf) {
                        Ok(size) => {
                            print!("Received {} bytes:\n{}", size, String::from_utf8_lossy(&buf[..size]));

                            match Request::try_from(&buf[..]) {
                                Ok(req) => {
                                    if req.path == "/home" {
                                        response = Response::new(
                                            StatusCode::Ok,
                                            Some("<h1>YAAAAAAYYYY!!!</h1> this is my home :D".to_string())
                                        )
                                    } else {
                                        response = Response::new(
                                            StatusCode::NotFound,
                                            Some("Hmmm I couldn't found anything :/".to_string())
                                        )
                                    }
                                },
                                Err(_) => {
                                    println!("failed to receive request");
                                    response = Response::new(
                                        StatusCode::BadRequest,
                                        None
                                    )
                                }
                            }

                            dbg!(&response);
                            println!("{}", &response);
                            let result = response.send(&mut stream);
                            if result.is_err() {
                                println!("There is an error!")
                            }
                        }
                        Err(err) => {
                            println!("===================================");
                            println!("[Server] Failed to read from connection: {}", err);
                            println!("===================================");
                            continue 'request_loop;
                        }
                    }
                }
                Err(err) => println!("failed to establish a connection: {}", err)
            }
        }
    }
}
