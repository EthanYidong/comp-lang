#[cfg(test)]
mod parser_tests;

mod token;
mod parser;
mod generator;

use clap::Clap;

use std::path::PathBuf;

#[derive(Clap)]
#[clap(version = "0.1", author = "Ethan <ethanyidong@gmail.com>")]
struct Opts {
    input_file: PathBuf,
    output_file: PathBuf,
}

fn main() {
    let opts: Opts = Opts::parse();

    let program_str = std::fs::read_to_string(opts.input_file).unwrap();
    let program = parser::program(&program_str).unwrap().1;
    
    let mut file = std::fs::File::create(opts.output_file).unwrap();
    generator::generate(program, &mut file);
}
