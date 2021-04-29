extern crate clap;
use clap::{App, Arg, SubCommand};
use std::env;
use std::process::exit;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .index(1),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .subcommand(
            SubCommand::with_name("get").about("Get value by key").arg(
                Arg::with_name("KEY")
                    .help("The string of the key")
                    .required(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set a key-value pair")
                .arg(
                    Arg::with_name("KEY")
                        .help("The string of the key")
                        .required(true),
                )
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string of the value")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a key-value pair")
                .arg(
                    Arg::with_name("KEY")
                        .help("The string of the key")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("get", Some(_sub_m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("set", Some(_sum_m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_sum_m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
