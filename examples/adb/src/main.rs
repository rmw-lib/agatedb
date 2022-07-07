use std::{path::PathBuf, sync::Arc};

use agatedb::{AgateOptions, ChecksumVerificationMode::NoVerification};
use anyhow::Result;
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

str!(db_path, ls, prefix, put, key, val);

macro_rules! get {
    ($matches:expr, $key:expr) => {
        $matches.get_one::<Box<[u8]>>($key)
    };
}

fn main() -> Result<()> {
    let matches = command!()
        .arg(arg!(<db_path> "agatedb database path").value_parser(value_parser!(PathBuf)))
        .subcommand(
            Command::new(LS)
                .about("list all keys-value")
                .arg(arg!([prefix] "list key-value where key match prefix")),
        )
        .subcommand(
            Command::new(PUT)
                .about("set key value")
                .arg(arg!(<key>))
                .arg(arg!(<val>)),
        )
        .get_matches();

    let db_path = matches.get_one::<PathBuf>(DB_PATH).unwrap();

    let db = Arc::new(
        AgateOptions {
            dir: db_path.clone(),
            checksum_mode: NoVerification,
            ..Default::default()
        }
        .open()?,
    );

    match matches.subcommand() {
        Some((LS, matches)) => match get!(matches, PREFIX) {
            Some(prefix) => {
                println!("{} {:?}", LS, prefix);
            }
            None => {
                println!("{}", LS);
            }
        },
        Some((PUT, matches)) => {
            let key = &get!(matches, KEY).unwrap()[..];
            let val = get!(matches, VAL).unwrap().clone();
            db.put(key, val)?;
        }
        None => {
            println!("OPEN {}", db_path.display().to_string());
            print!("> ");
        }
        _ => {
            unreachable!("unkown cmd");
        }
    }
    Ok(())
}
