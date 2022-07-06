use std::path::PathBuf;

use agatedb::{AgateOptions, ChecksumVerificationMode::NoVerification};
use clap::{arg, command, value_parser, Command};
use paste::paste;

macro_rules! str {
  ($str:ident) => {
    paste! {
      const [<$str:upper>]: &'static str = stringify!($str);
    }
  };
  ($($str:ident),+) => {
    $(str!($str);)+
  };
}

str!(db_path, ls, table, set);

fn main() {
    let matches = command!()
        .arg(arg!(<DB_PATH> "agatedb database path").value_parser(value_parser!(PathBuf)))
        .subcommand(
            Command::new(LS)
                .about("list all keys-value")
                .arg(arg!([PREFIX] "list key-value where key match prefix")),
        )
        .subcommand(
            Command::new(SET)
                .about("set key value")
                .arg(arg!(<key>))
                .arg(arg!(<value>)),
        )
        .get_matches();

    let db_path = matches.get_one::<PathBuf>(DB_PATH).unwrap();

    let db = AgateOptions {
        dir: db_path.clone(),
        checksum_mode: NoVerification,
        ..Default::default()
    }
    .open();

    match matches.subcommand() {
        Some((LS, matches)) => match matches.get_one::<String>(TABLE) {
            Some(prefix) => {
                println!("{} {:?}", LS, prefix);
            }
            None => {
                println!("{}", LS);
            }
        },
        None => {
            print!("> ");
        }
        _ => {
            unreachable!("unkown cmd");
        }
    }
    //println!("{}", db_path);
}
