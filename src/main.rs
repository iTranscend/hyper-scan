use std::io::Write;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use structopt::StructOpt;
// use std::process::Command;

#[derive(Debug, StructOpt)]
#[structopt(name = "turbo-scanner-rs", about = "A simple TCP port scanner")]

struct Opts {
    #[structopt(short = "h", long = "host", default_value = "localhost")]
    host: String,
}

fn main() {
    let opts = Opts::from_args();
    println!("Host to scan: {:?}", opts.host);
    //convert host to socket address
    let addr: SocketAddr = opts.host.to_socket_addrs().unwrap().next().unwrap();

    // connect to the server
    let mut socket = match TcpStream::connect(&addr) {
        Ok(s) => s,
        Err(e) => {
            println!("Could not connect to server: {}", e);
            return;
        }
    };
    println!("Connected to server");

    // send the message to the server
    let mut msg = String::new();
    msg.push_str("Yall fall for anytihing, kingpin you're a pin king!");
    let mut bytes_sent = 0;
    while bytes_sent < msg.len() {
        let bytes_sent_now = socket.write(&msg.as_bytes()[bytes_sent..]);
        match bytes_sent_now {
            Ok(n) => {
                bytes_sent += n;
                println!("Sent {} bytes", n);
            }
            _ => {
                println!("Could not send message");
                return;
            }
        }
    }
}
