use clap::Parser;
use std::str;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    bytes: Option<String>,
    #[arg(short, long)]
    sep: Option<String>,
}

fn main() {
    let args = Args::parse();

    let sep = match args.sep {
        Some(s) => s,
        None => " ".to_string(),
    };

    let mut bs: Vec<u8> = Vec::new();

    for i in args.bytes.unwrap().split(&sep) {
        let b = match i.parse::<u8>() {
            Ok(x) => x,
            Err(e) => {
                eprintln!("not u8: {:?}, error: {:?}", i, e);
                std::process::exit(1);
            }
        };

        bs.push(b);
    }

    println!("{}", str::from_utf8(&bs).unwrap());
}
