use std::path::PathBuf;

use agatedb::{AgateOptions, ChecksumVerificationMode::NoVerification};
use clap::{arg, command, value_parser};

fn main() {
    let matches = command!()
        .arg(arg!(<DB_PATH> "agatedb database path").value_parser(value_parser!(PathBuf)))
        .arg(arg!([CMD] "cmd"))
        .get_matches();

    let db_path = matches.get_one::<PathBuf>("DB_PATH").unwrap();

    let db = AgateOptions {
        dir: db_path.clone(),
        checksum_mode: NoVerification,
        ..Default::default()
    }
    .open();

    match matches.get_one::<String>("CMD") {
        Some(cmd) => {
            println!("{}", cmd);
        }
        None => {
            print!("> ");
        }
    }
    //println!("{}", db_path);
}
