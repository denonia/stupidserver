mod threads;

use std::{
    io, 
    net::TcpListener
};

use server::handle_conn;
use threads::ThreadPool;

const LISTENER_ADDR: &str = "127.0.0.1:80";

fn main() -> io::Result<()> {

    let listener = TcpListener::bind(LISTENER_ADDR)?;

    println!(
        "Listening at {}....",
        LISTENER_ADDR
    );

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => pool.execute(|| handle_conn(stream).unwrap()),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
