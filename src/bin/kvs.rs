use kvs::{KvStore, KvsError, Result};
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
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let kvpath = current_dir.as_path();
    let opt = Opt::from_args();
    match opt.cmd {
        Command::Get { key } => {
            let mut store = KvStore::open(kvpath)?;
            if let Some(value) = store.get(key)? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Command::Set { key, value } => {
            let mut store = KvStore::open(kvpath)?;
            store.set(key, value)?;
        }
        Command::Rm { key } => {
            let mut store = KvStore::open(kvpath)?;
            match store.remove(key) {
                Ok(()) => {}
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(e) => return Err(e),
            }
        }
    }
    Ok(())
}
