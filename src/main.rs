use futures::stream::{self, StreamExt};
use futures::TryStreamExt;
use std::num::{NonZeroU64, NonZeroUsize};
use std::time::Instant;
use structopt::StructOpt;
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio::time;

struct Port {
    num: u16,
    open: bool,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "hyper-scan",
    about = "A multi-threaded TCP port scanner and service detection utility."
)]
struct Opts {
    /// Host to scan
    #[structopt(short = "h", long = "host", default_value = "127.0.0.1")]
    host: String,

    /// Print verbose output
    #[structopt(short = "v", long = "verbose", help = "Print verbose output")]
    verbose: bool,

    /// Number of threads to use
    #[structopt(short = "j", long = "threads", help = "Number of threads to use")]
    threads: Option<NonZeroUsize>,

    /// Port to begin scanning from
    #[structopt(
        short = "s",
        long = "startPort",
        help = "Port to start scanning from",
        default_value = "1"
    )]
    start_port: u16,

    /// Port to end scanning at
    #[structopt(
        short = "e",
        long = "endPort",
        help = "Port to end scanning at",
        default_value = "65535"
    )]
    end_port: u16,

    /// Number of seconds to wait before timing out on a port check (ms)
    #[structopt(
        short = "t",
        long = "timeout",
        help = "Number of seconds to wait before timing out of a port check (ms).",
        default_value = "3000"
    )]
    timeout: NonZeroU64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let opts = Opts::from_args();

    let threads = match opts.threads {
        Some(threads) => threads,
        None => std::thread::available_parallelism()?,
    }
    .get();
    let timeout = opts.timeout.get();

    if opts.verbose {
        println!("Host to scan: {:?}", opts.host);
        println!("Number of threads: {:?}", threads);
        println!("Timeout: {:?}ms", timeout);
    }

    let mut open_ports = stream::iter(opts.start_port..=opts.end_port)
        .map(move |num| {
            let mut port = Port { num, open: false };

            let host = opts.host.clone();
            async move {
                // retry 3 times
                for _ in 1..=3 {
                    tokio::select! {
                        _ = async {
                            if let Ok(_) = TcpStream::connect((host.as_str(), port.num)).await {
                                println!("Port {} is open", port.num);
                                port.open = true;
                            }
                        } => break,
                        _ = time::sleep(time::Duration::from_millis(timeout)) => {
                            println!("Port {} check timed out", port.num);
                        },
                    };
                }
                port
            }
        })
        .map(tokio::spawn)
        .buffer_unordered(threads);

    while let Some(port) = open_ports.try_next().await? {
        if port.open {
            inspect_port(port.num).await;
        }
    }

    let elapsed = now.elapsed();
    println!("Time Elapsed: {:?}", elapsed);
    Ok(())
}

async fn inspect_port(port: u16) {
    let output = Command::new("lsof")
        .arg(format!("-i:{port}"))
        .kill_on_drop(true)
        .output()
        .await;

    let stdout = String::from_utf8_lossy(match &output {
        Ok(output) => &output.stdout,
        Err(_) => &[],
    });
    println!("{}", stdout);
}
