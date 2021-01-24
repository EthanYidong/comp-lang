#[cfg(test)]
mod parser_tests;

mod token;
mod parser;
mod generator;

fn main() {
    let program_str = std::fs::read_to_string("test.comp").unwrap();
    let program = parser::program(&program_str).unwrap().1;
    
    let mut file = std::fs::File::create("test.cpp").unwrap();
    generator::generate(program, &mut file)
}
