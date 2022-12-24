use clap::Parser;
use std::str;

/// Bytes & String type bidirectional conversion
#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    mode: Mode,
}

#[derive(Parser, Debug)]
enum Mode {
    /// Mode: string to bytes
    S(Sargs),
    /// Mode: bytes to string
    B(Bargs),
}

trait Printer {
    fn print(&self) {}
}

#[derive(Parser, Debug)]
struct Sargs {
    /// String words
    string: Option<String>,
}

impl Printer for Sargs {
    fn print(&self) {
        println!("hello sargs");
    }
}

#[derive(Parser, Debug)]
struct Bargs {
    /// Bytes vector words
    #[arg(short, long)]
    bytes: Option<String>,
    /// Separator between vector words
    #[arg(short, long)]
    sep: Option<String>,
}

impl Printer for Bargs {
    fn print(&self) {
        let args = Bargs::parse();

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
}

fn main() {
    let opts: Opts = Opts::parse();
    let p: Box<dyn Printer> = match opts.mode {
        Mode::S(s) => Box::new(s),
        Mode::B(b) => Box::new(b),
    };
    p.print();
}
