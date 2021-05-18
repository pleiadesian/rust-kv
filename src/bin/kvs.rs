use kvs::{KvStore, Result};
use std::env;
use std::path::Path;
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
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() -> Result<()> {
    let kvpath = Path::new("/tmp/rust-kv");
    let opt = Opt::from_args();
    match opt.cmd {
        Command::Get { key: key } => {}
        Command::Set { key: key, value: value } => {
            let mut store = KvStore::open(kvpath)?;
            store.set(key, value)?;
        }
        Command::Rm { key: key } => {
            let mut store = KvStore::open(kvpath)?;
            store.remove(key)?;
        }
    }
    Ok(())
}
