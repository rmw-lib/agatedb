use std::{fmt::format, path::PathBuf, sync::Arc};

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

str!(db_path, ls, prefix, set, key, val, get);

fn display(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(s) => format!("{:?}", s),
        _ => format!("{:?}", bytes),
    }
}

macro_rules! get {
    ($matches:expr, $key:expr) => {{ $matches.get_one::<String>($key) }};
}

fn cmd(cmd: Command) -> Command {
    cmd.subcommand(
        Command::new(SET)
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
}

fn main() -> Result<()> {
    let matches = cmd(command!()
        .arg(arg!(<db_path> "agatedb database path").value_parser(value_parser!(PathBuf))))
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
            (SET, matches) => {
                let key = get!(matches, KEY).unwrap().as_bytes();
                let val = get!(matches, VAL).unwrap().as_bytes().to_vec();
                db.set(key, val)?;
            }

            (GET, matches) => {
                let key = get!(matches, KEY).unwrap().as_bytes();
                let value = db.get(key)?;
                dbg!(&value);
                let value = if value.version == 0 {
                    "None".into()
                } else {
                    display(&value.value)
                };
                println!("\n{} → {}", display(key), value);
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
