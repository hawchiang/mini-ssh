use std::{io::Read, net::TcpStream};

use clap::{arg, Command};
use ssh2::Session;

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    let matches = Command::new("mini-ssh")
        .arg(arg!(-n --name <NAME>))
        .arg(arg!(-h --host <HOST>))
        .arg(arg!(-p --password <PASSWORD>))
        .arg(arg!(-e --exec <EXEC>))
        .get_matches();
    let host = matches.get_one::<String>("host").unwrap();
    let name = matches.get_one::<String>("name").unwrap();
    let password = matches.get_one::<String>("password").unwrap();
    let cmd = matches.get_one::<String>("exec").unwrap();
    let tcp = TcpStream::connect(host).unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    session.userauth_password(name, password).unwrap();
    let mut channel = session.channel_session().unwrap();
    channel.exec(cmd).unwrap();
    let mut buf = String::new();
    channel.read_to_string(&mut buf).unwrap();
    channel.wait_close().unwrap();
    println!("{}", buf);
}
