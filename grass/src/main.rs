use structopt::StructOpt;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    pattern: String,

    #[structopt(short, long)]
    line_num: bool,
}

fn main() {
    let args = Cli::from_args();
    let line_printer = LinePrinter { show_line_number: args.line_num };
    match File::open(&args.path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut done_flag = Ok(1);
            let mut line_count = 0;

            while !is_done(&done_flag) {
                let mut line = String::new();
                done_flag = reader.read_line(&mut line);
                if is_done(&done_flag) { break; }
                line_count = line_count + 1;
                if line.contains(&args.pattern) {
                    line_printer.print(&line, &line_count)
                }
            }
        }
        Err(err) => {
            println!("Could not read file - {}", err);
            std::process::exit(1);
        }
    }
}

struct LinePrinter {
    show_line_number: bool
}

impl LinePrinter {
    fn print(&self, line: &str, line_num: &i32) {
        if self.show_line_number {
            print!("{0}: {1}", line_num, line);
        } else {
            print!("{}", line);
        }
    }
}

fn is_done(done: &Result<usize, std::io::Error>) -> bool {
    match done {
        Ok(val) => {
            return *val == 0;
        }
        Err(_) => {
            return true;
        }
    }
}