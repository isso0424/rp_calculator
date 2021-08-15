use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

// Command line arguments
#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

// Calculator
struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    // TODO: calculate logic
    pub fn eval(&self, formula: &str) -> i32 {
        0
    }
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        run(reader, opts.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

// Command runner
// R: RPN format formula stream
//   ex: Text file, Standard Input, etc...
fn run<R: BufRead>(reader: R, verbose: bool) {
    let calculator = RpnCalculator::new(verbose);
    for line in reader.lines() {
        if let Ok(line) = line {
            let ans = calculator.eval(&line);
            println!("{}", ans);
        } else if let Err(err) = line {
            eprintln!("{}", err);
        }
    }
}
