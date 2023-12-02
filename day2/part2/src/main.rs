use std::io;
use std::cmp::max;

use nom::{
    self,
    IResult,
    bytes::complete::{tag, take_while1},
    sequence::{separated_pair, delimited, preceded},
    character::{is_digit, complete::{digit1, not_line_ending, line_ending}},
    branch::alt,
    error::Error,
    multi::{fold_many0, separated_list0},
};

fn parse_color(input: &str) -> IResult<&str, &str> {
    alt((tag("green"), tag("red"), tag("blue")))(input)
}

fn parse_number_and_color(input: &str) -> IResult<&str, (u32, &str)> {
    let (rest, (number_str, color)) = separated_pair(digit1, tag(" "), parse_color)(input)?;
    let number = u32::from_str_radix(number_str, 10).unwrap();
    Ok( (rest, (number, color)) )
}

fn get_game_power(input: &str) -> IResult<&str, (u32, u32)> {
    let mut min_red = 0u32;
    let mut min_green = 0u32;
    let mut min_blue = 0u32;

    let (mut rest, game_id_str) = delimited(tag("Game "), digit1, tag(": "))(input)?;
    let game_id = u32::from_str_radix(game_id_str, 10).unwrap();

    loop {
        let number: u32;
        let color: &str;
        (rest, (number, color)) = parse_number_and_color(rest)?;
        if color == "red" { min_red = max(min_red, number); }
        if color == "green" { min_green = max(min_green, number); }
        if color == "blue" { min_blue = max(min_blue, number); }

        match alt((tag::<&str, &str, Error<&str>>(", "), tag::<&str, &str, Error<&str>>("; ")))(rest) {
            Ok( (new_rest, _) ) => {
                rest = new_rest;
            },
            Err(_) => {
                return Ok( (rest, (game_id, min_red * min_green * min_blue)) );
            }
        }
    }
}

fn main() {
    let mut game_power_sum = 0u32;
    let stdin = io::stdin();

    for line in stdin.lines() {
        let (_, (_, game_power)) = get_game_power(line.unwrap().as_str()).unwrap();
        game_power_sum += game_power;
    }

    println!("{}", game_power_sum);
}
