use std::env;
use std::process::exit;
use structopt::StructOpt;
use std::path::Path;
use kvs::{KvStore, Result};

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

fn main() -> Result<()> {
    let kvpath = Path::new("/tmp/rust-kv");
    let opt = Opt::from_args();
    match opt.cmd {
        Command::Get { _key } => {
            
        }
        Command::Set { _key, _value } => {            
            let mut store = KvStore::open(kvpath)?;
            store.set(_key, _value)?;
        }
        Command::Rm { _key } => {

        }
    }
    Ok(())
}
