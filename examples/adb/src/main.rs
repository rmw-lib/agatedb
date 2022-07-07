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

str!(db_path, ls, prefix, put, key, val, get);

macro_rules! get {
    ($matches:expr, $key:expr) => {{ $matches.get_one::<String>($key) }};
}

fn main() -> Result<()> {
    let matches = command!()
        .arg(arg!(<db_path> "agatedb database path").value_parser(value_parser!(PathBuf)))
        .subcommand(
            Command::new(PUT)
                .about("set key value")
                .arg(arg!(<key>))
                .arg(arg!(<val>)),
        )
        .subcommand(Command::new(GET).about("get key value").arg(arg!(<key>)))
        .subcommand(
            Command::new(LS)
                .about("list all keys-value")
                .arg(arg!([prefix] "list key-value where key match prefix")),
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

    if let Some(sub) = matches.subcommand() {
        match sub {
            (LS, matches) => match get!(matches, PREFIX) {
                Some(prefix) => {
                    println!("{} {:?}", LS, prefix);
                }
                None => {
                    println!("{}", LS);
                }
            },
            (PUT, matches) => {
                let key = get!(matches, KEY).unwrap().as_bytes();
                let val = get!(matches, VAL).unwrap().as_bytes().to_vec();
                db.put(key, val)?;
            }

            (GET, matches) => {
                let key = get!(matches, KEY).unwrap().as_bytes();
                let value = db.get(key)?;
                dbg!(&value);
                let key = match std::str::from_utf8(key) {
                    Ok(k) => k.into(),
                    _ => format!("{:?}", key),
                };
                println!("\n{} → {:?}", key, value.value)
            }
            _ => {
                unreachable!("unkown cmd");
            }
        }
    } else {
        println!("OPEN {}", db_path.display().to_string());
        print!("> ");
    }
    Ok(())
}
