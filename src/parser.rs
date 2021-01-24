use nom::{
    IResult, 
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, not_line_ending, space0, space1, char as match_char},
    combinator::{all_consuming, map, map_parser, rest},
    branch::alt,
    sequence::{pair, preceded, terminated},
    multi::{separated_list0, many0_count},
};
use crate::token::*;

pub fn program(input: &str) -> IResult<&str, Program> {
    let (input, lines) = separated_list0(line_ending, line)(input)?;
    let program = Program {
        lines: lines
    };
    Ok((input, program))
}

pub fn line(input: &str) -> IResult<&str, Line> {
    let (input, (indentation, code)) = map_parser(not_line_ending, pair(indentation, code))(input)?;
    let line = Line {
        indentation,
        code,
    };

    Ok((input, line))
}

pub fn indentation(input: &str) -> IResult<&str, usize> {
    many0_count(alt((tag("\t"), tag("    "))))(input)
}

pub fn code(input: &str) -> IResult<&str, Code> {
    alt((code_empty, code_special, code_raw))(input)
}

pub fn code_empty(input: &str) -> IResult<&str, Code> {
    map(all_consuming(space0), |_| Code::Empty)(input)
}

pub fn code_special(input: &str) -> IResult<&str, Code> {
    map(preceded(match_char('~'), special), Code::Special)(input)
}

pub fn special(input: &str) -> IResult<&str, Special> {
    all_consuming(terminated(alt((special_env, special_input, special_main, special_control, special_escaped)), space0))(input)
}

pub fn special_env(input: &str) -> IResult<&str, Special> {
    map(preceded(pair(tag("env"), space1), rest), |s| Special::Env(String::from(s)))(input)
}

pub fn special_input(input: &str) -> IResult<&str, Special> {
    map(preceded(pair(tag(">"), space0), rest), |s| Special::Input(String::from(s)))(input)
}

pub fn special_control(input: &str) -> IResult<&str, Special> {
    map(control, Special::Control)(input)
}

pub fn special_escaped(input: &str) -> IResult<&str, Special> {
    map(preceded(pair(tag("!"), space0), rest), |s| Special::Escaped(String::from(s)))(input)
}


pub fn control(input: &str) -> IResult<&str, Control> {
    alt((control_if, control_range, control_for, control_while, control_loop))(input)
}

pub fn control_if(input: &str) -> IResult<&str, Control> {
    map(preceded(pair(tag("if"), space1), rest), |s| Control::If(String::from(s)))(input)
}

pub fn control_range(input: &str) -> IResult<&str, Control> {
    map(preceded(pair(tag("range"), space1), comma_list), |l| Control::Range(String::from(l[0]), String::from(l[1]), String::from(l[2])))(input)
}

pub fn control_for(input: &str) -> IResult<&str, Control> {
    map(preceded(pair(tag("for"), space1), comma_list), |l| Control::For(String::from(l[0]), String::from(l[1]), String::from(l[2])))(input)
}

pub fn control_while(input: &str) -> IResult<&str, Control> {
    map(preceded(pair(tag("while"), space1), rest), |s| Control::While(String::from(s)))(input)
}

pub fn control_loop(input: &str) -> IResult<&str, Control> {
    map(tag("loop"), |_| Control::Loop)(input)
}

pub fn special_main(input: &str) -> IResult<&str, Special> {
    map(tag("main"), |_| Special::Main)(input)
}

pub fn code_raw(input: &str) -> IResult<&str, Code> {
    Ok(("", Code::Raw(String::from(input))))
}

fn comma_list(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(pair(tag(","), space0), alt((take_until(","), rest)))(input)
}
