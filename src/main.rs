use clap::Parser;
use std::str;

#[derive(Parser, Debug)]
struct Args {
    string: Option<String>,
    sep: Option<String>,
}

fn main() {
    let args = Args::parse();

    let sep = match args.sep {
        Some(s) => s,
        None => " ".to_string(),
    };

    let mut bs: Vec<u8> = Vec::new();

    for i in args.string.unwrap().split(&sep) {
        let b = match i.parse::<u8>() {
            Ok(x) => x,
            Err(e) => {
                eprintln!("not u8: {:?}, error: {:?}", i, e);
                std::process::exit(1);
            }
        };
        // println!("b: {:?}", b);

        bs.push(b);
    }

    // println!("{:?}", &bs);

    println!("{}", str::from_utf8(&bs).unwrap());
}
