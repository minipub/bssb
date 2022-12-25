use clap::{Parser, Subcommand};
use std::str;

#[derive(Parser, Debug)]
#[clap(name = "bssb", version = "0.1.1")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Mode: string to bytes
    S2b(Sargs),
    /// Mode: bytes to string
    B2s(Bargs),
}

trait Printer {
    fn print(&self) {}
}

#[derive(Parser, Debug)]
struct Sargs {
    /// String words
    #[clap(short, long)]
    words: String,
    /// Separator between string words
    #[clap(short, long)]
    sep: Option<String>,
}

impl Printer for Sargs {
    fn print(&self) {
        let sep = match &self.sep {
            Some(s) => s,
            None => " ",
        };

        let bs = self.words.clone().into_bytes();

        let ss = bs.into_iter().map(|x| x.to_string());

        let vs: Vec<String> = ss.rev().collect();

        println!("{}", vs.join(sep));
    }
}

#[derive(Parser, Debug)]
struct Bargs {
    /// Bytes vector
    #[clap(short, long)]
    bytes: String,
    /// Separator between bytes vector
    #[clap(short, long)]
    sep: Option<String>,
}

impl Printer for Bargs {
    fn print(&self) {
        let sep = match &self.sep {
            Some(s) => s,
            None => " ",
        };

        let mut bs: Vec<u8> = Vec::new();

        for i in self.bytes.split(&sep) {
            let b = match i.parse::<u8>() {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("not u8, ele: {:?}, sep: {:?}, error: {:?}", i, sep, e);
                    std::process::exit(1);
                }
            };

            bs.push(b);
        }

        println!("{}", str::from_utf8(&bs).unwrap());
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    // println!("opts: {:?}", opts);

    let p: Box<dyn Printer> = match opts.subcmd {
        SubCommand::S2b(s) => Box::new(s),
        SubCommand::B2s(b) => Box::new(b),
    };
    p.print();
}
