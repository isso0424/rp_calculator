use anyhow::{Context, Result};
use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use thiserror::Error;

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

#[derive(Error, Debug)]
enum InputError {
    #[error("Unknown operator: {0}")]
    UnknownOperator(String),
    #[error("Invalid syntax: {0}")]
    SyntaxError(String),
}

// Calculator
struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().ok_or(InputError::SyntaxError(
                    "Cannot fetch next number".to_string(),
                ))?;
                let x = stack.pop().ok_or(InputError::SyntaxError(
                    "Cannot fetch next number".to_string(),
                ))?;
                let res = match token {
                    "+" => Ok(x + y),
                    "-" => Ok(x - y),
                    "*" => Ok(x * y),
                    "/" => Ok(x / y),
                    invalid => Err(InputError::UnknownOperator(invalid.to_string())),
                }?;
                stack.push(res);
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            Ok(stack[0])
        } else {
            Err(InputError::SyntaxError(
                "Value remains on the stack".to_string(),
            ))?
        }
    }
}

fn read_from_file(path: &str) -> Result<File> {
    File::open(path).with_context(|| format!("failed to open the file from {}", path))
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        match read_from_file(&path) {
            Ok(f) => {
                let reader = BufReader::new(f);
                run(reader, opts.verbose);
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
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
            match calculator.eval(&line) {
                Ok(ans) => println!("{}", ans),
                Err(e) => eprintln!("{}", e),
            };
        } else if let Err(err) = line {
            eprintln!("{}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("-5").unwrap(), -5);

        assert_eq!(calc.eval("2 3 +").unwrap(), 5);
        assert_eq!(calc.eval("2 3 -").unwrap(), -1);
        assert_eq!(calc.eval("2 3 *").unwrap(), 6);
        assert_eq!(calc.eval("4 2 /").unwrap(), 2);
    }
}
