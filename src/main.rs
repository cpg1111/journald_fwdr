extern crate docopt;
extern crate serde;
extern crate tokio;

use std::env;

use docopt::Docopt;
use serde::Deserialize;
use tokio::net::unix;
use tokio::runtime;
use tokio::task::JoinHandle;

const USAGE: &'static str = "
Usage: journald_fwdr [options]
       journald_fwdr (--help | --version)
Options:
    -h, --help           Show this message
    -v, --version        Show version
    -s, --journal-socket Absolute path to journal socket
    -c, --client-socket  Absolute path to client socket
    -a, --syslog-addr    Host and Port to remote syslog server
";

#[derive(Clone, Deserialize)]
struct Args {
    flag_help: bool,
    flag_version: bool,
    flag_journal_socket: String,
    flag_client_socket: String,
    flag_syslog_addr: String,
}

fn default_journal_socket(args: Args) -> Args {
    if args.flag_journal_socket.len() == 0 {
        return Args{
            flag_help: args.flag_help,
            flag_version: args.flag_version,
            flag_journal_socket: String::from("/usr/lib/systemd/system/systemd-journald.socket"),
            flag_client_socket: args.flag_client_socket,
            flag_syslog_addr: args.flag_syslog_addr,
        }
    }
    return args
}

fn default_client_socket(args: Args) -> Args {
    if args.flag_journal_socket.len() == 0 {
        return Args{
            flag_help: args.flag_help,
            flag_version: args.flag_version,
            flag_journal_socket: args.journal_socket,
            flag_client_socket: String::from("/var/run/journald_fwdr.sock"),
            flag_syslog_addr: args.flag_syslog_addr,
        }
    }
    return args
}

fn default_syslog_addr(args: Args) -> Args {
    if args.flag_journal_socket.len() == 0 {
        return Args{
            flag_help: args.flag_help,
            flag_version: args.flag_version,
            flag_journal_socket: args.journal_socket,
            flag_client_socket: args.client_socket,
            flag_syslog_addr: String::from("127.0.0.1:541"),
        }
    }
    return args
}

fn main() {
    let local_sock = unix::UnixDatagram::bind()
    let mut rt = runtime::Runtime::new().unwrap();
    rt.spawn()
}
