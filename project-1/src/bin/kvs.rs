use std::env;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    Get { _key: String },
    Set { _key: String, _value: String },
    Rm { _key: String },
}

fn main() {
    let opt = Opt::from_args();
    match opt.cmd {
        Command::Get { _key } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Command::Set { _key, _value } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Command::Rm { _key } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
