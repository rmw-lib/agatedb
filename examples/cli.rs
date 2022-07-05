use std::path::PathBuf;

use agatedb::{AgateOptions, ChecksumVerificationMode::NoVerification};
use clap::{arg, command, value_parser, Command};

fn main() {
  let matches = command!()
    .arg(arg!(<DB_PATH> "agatedb database path").value_parser(value_parser!(PathBuf)))
    .subcommand(Command::new("ls").about("list all table"))
    .get_matches();

    let db_path = matches.get_one::<PathBuf>("DB_PATH").unwrap();

    let db = AgateOptions {
      dir: db_path.clone(),
      checksum_mode: NoVerification,
      ..Default::default()
    }
    .open();

    match matches.subcommand() {
      Some(("ls", sub_matches)) => {
        println!("{}", cmd);
      }
      None => {
        print!("> ");
      }
      _=>{
        unreachable!("unkown cmd"),
      }
    }
    //println!("{}", db_path);
}
