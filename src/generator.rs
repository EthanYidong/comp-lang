use std::io::Write;

use crate::token::*;

pub fn generate(program: Program, output: &mut impl Write) {
    let mut current_indentation = 0;

    for (idx, line) in program.lines.iter().enumerate() {
        match &line.code {
            Code::Empty => {},
            _ => {
                if line.indentation > current_indentation + 1 {
                    panic!("Indentation error: line {}", idx + 1);
                }
                while line.indentation > current_indentation {
                    writeln!(output, "{}{{", "\t".repeat(current_indentation)).unwrap();
                    current_indentation += 1;
                }
                while line.indentation < current_indentation {
                    current_indentation -= 1;
                    writeln!(output, "{}}}", "\t".repeat(current_indentation)).unwrap();
                }
            },
        }

        write!(output, "{}", "\t".repeat(current_indentation)).unwrap();
        match &line.code {
            Code::Empty => {
                writeln!(output, "").unwrap();
            },
            Code::Raw(raw_code) => {
                writeln!(output, "{};", raw_code).unwrap();
            },
            Code::Special(special) => {
                match special {
                    Special::Env(env_name) => {
                        match env_name.as_str() {
                            "usaco" => writeln!(output, r##"#include <bits/stdc++.h>"##).unwrap(),
                            _ => panic!("Invalid env name {}: line {}", env_name, idx + 1)
                        }
                    },
                    Special::Input(input_name) => {
                        writeln!(output, "std::cin >> {};", input_name).unwrap();
                    },
                    Special::Control(control) => {
                        match control {
                            Control::If(clause) => {
                                writeln!(output, "if ({})", clause).unwrap();
                            },
                            Control::Range(id, start, len) => {
                                writeln!(output, "for (int {0} = {1}; {0} < {1} + {2}; {0}++)", id, start, len).unwrap();
                            },
                            Control::For(init, clause, post) => {
                                writeln!(output, "for ({}; {}; {})", init, clause, post).unwrap();
                            },
                            Control::While(clause) => {
                                writeln!(output, "while ({})" , clause).unwrap();
                            }
                            Control::Loop => {
                                writeln!(output, "while (true)").unwrap();
                            }
                        }
                    },
                    Special::Main => {
                        writeln!(output, "int main()").unwrap();
                    },
                    Special::Escaped(escaped) => {
                        writeln!(output, "{}", escaped).unwrap();
                    }
                }
            }
        }
    }
    while 0 < current_indentation {
        current_indentation -= 1;
        writeln!(output, "{}}}", "\t".repeat(current_indentation)).unwrap();
    }
}