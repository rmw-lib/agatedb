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

str!(db_path, ls, table);

fn main() {
    let matches = command!()
        .arg(arg!(<DB_PATH> "agatedb database path").value_parser(value_parser!(PathBuf)))
        .subcommand(
            Command::new(LS)
                .about("list all table")
                .arg(arg!([TABLE] "list key value in table")),
        )
        .get_matches();

    let db_path = matches.get_one::<PathBuf>(DB_PATH).unwrap();

    let db = AgateOptions {
        dir: db_path.clone(),
        create_if_not_exists: true,
        checksum_mode: NoVerification,
        ..Default::default()
    }
    .open();

    match matches.subcommand() {
        Some((LS, matches)) => match matches.get_one::<String>(TABLE) {
            Some(table) => {
                println!("{} {:?}", LS, table);
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
