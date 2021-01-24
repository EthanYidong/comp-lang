use crate::*;
use crate::token::*;

#[test]
fn parse_code() {
    assert_eq!(parser::code("hello"), Ok(("", Code::Raw(String::from("hello")))));

    assert_eq!(parser::code("\t  \t "), Ok(("", Code::Empty)));
}

#[test]
fn parse_line() {
    assert_eq!(parser::line("\t\thello"), Ok(("", Line {
        indentation: 2,
        code: Code::Raw(String::from("hello"))
    })));

    assert_eq!(parser::line("    hello world"), Ok(("", Line {
        indentation: 1,
        code: Code::Raw(String::from("hello world"))
    })));
}

#[test]
fn parse_program() {
    let expected_output = Program {
        lines: vec![
            Line {
                indentation: 1,
                code: Code::Raw(String::from("hello "))
            },
            Line {
                indentation: 0,
                code: Code::Empty,
            },
            Line {
                indentation: 0,
                code: Code::Raw(String::from("test"))
            },
        ]
    };
    assert_eq!(parser::program("\thello \n  \t  \ntest"), Ok(("", expected_output)));
}
