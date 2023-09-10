use std::net::{SocketAddr, SocketAddrV4};
use std::str::FromStr;
use structopt::StructOpt;
use udt::*;

#[derive(Debug, StructOpt)]
struct CliArgs {
    /// The input file default to stdin
    #[structopt(short = "i", long = "input", default_value = "/dev/stdin")]
    input: std::path::PathBuf,
    /// Server address (no flag)
    #[structopt(name = "ADDRESS")]
    address: String,
    /// Server port (no flag)
    #[structopt(name = "PORT")]
    port: u16,
}

fn main() {
    let localhost = std::net::Ipv4Addr::from_str("0.0.0.0").unwrap();
    let sock = UdtSocket::new(SocketFamily::AFInet, SocketType::Stream).unwrap();
    sock.setsockopt(UdtOpts::UDP_SNDBUF, 64).unwrap();
    sock.setsockopt(UdtOpts::UDP_RCVBUF, 64).unwrap();
    sock.bind(SocketAddr::V4(SocketAddrV4::new(localhost, 0)))
        .unwrap();
    let args = CliArgs::from_args();
    let mut input = std::fs::File::open(args.input).unwrap();

    sock.connect(SocketAddr::V4(SocketAddrV4::new(
        std::net::Ipv4Addr::from_str(&args.address).unwrap(),
        args.port,
    )))
    .unwrap();

    eprintln!("Connected to {}", sock.getpeername().unwrap());

    let mut buf = [0u8; 1024];
    loop {
        let n = std::io::Read::read(&mut input, &mut buf).unwrap();
        if n == 0 {
            break;
        }
        sock.send(&buf[..n]).unwrap();
    }

    sock.close().unwrap();
}
