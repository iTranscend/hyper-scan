use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::num::NonZeroUsize;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "turbo-scanner-rs", about = "A simple TCP port scanner")]

struct Opts {
    #[structopt(short = "h", long = "host", default_value = "localhost")]
    host: String,

    #[structopt(short = "v", long = "verbose", help = "Print verbose output")]
    verbose: bool,

    #[structopt(short = "t", long = "threads", help = "Number of threads to use")]
    threads: Option<NonZeroUsize>,

    #[structopt(short = "sp", long = "startPort", help = "Port to start scanning from")]
    start_port: Option<u32>,

    #[structopt(short = "ep", long = "endPort", help = "Port to end scanning at")]
    end_port: Option<u32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();

    let available_threads = std::thread::available_parallelism()?;
    let threads = opts.threads.unwrap_or(available_threads).get();

    if opts.verbose {
        println!("Host to scan: {:?}", opts.host);
        println!("Number of threads: {:?}", threads);
    }

    let mut open_ports: Vec<u32> = vec![];

    for port in opts.start_port.unwrap_or(100)..opts.end_port.unwrap_or(145) {
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

    Ok(())
}
