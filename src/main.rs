use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "turbo-scanner-rs", about = "A simple TCP port scanner")]

struct Opts {
    #[structopt(short = "h", long = "host", default_value = "localhost")]
    host: String,
}

fn main() {
    let opts = Opts::from_args();
    println!("Host to scan: {:?}", opts.host);

    let mut open_ports: Vec<u32> = vec![];

    for port in 0..65535 {
        let pre_addr = format!("{}:{}", opts.host, port);
        let addr: SocketAddr = pre_addr.to_socket_addrs().unwrap().next().unwrap();
        match TcpStream::connect(&addr) {
            Ok(_) => open_ports.push(port),
            Err(_) => continue,
        }
    }

    println!("Open ports: {:?}", &open_ports);

    for port in open_ports {
        let output = Command::new("lsof")
            .arg(format!("-i:{port}"))
            .output()
            .expect("lsof falied to execute");

        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    }
}
