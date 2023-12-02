use std::io;

use nom::{
    self,
    IResult,
    bytes::complete::{tag, take_while1},
    sequence::{separated_pair, delimited, preceded},
    character::{is_digit, complete::{digit1, not_line_ending, line_ending}},
    branch::alt,
    error::Error,
    multi::{fold_many0, separated_list0};
};

fn parse_color(input: &str) -> IResult<&str, &str> {
    alt((tag("green"), tag("red"), tag("blue")))(input)
}

fn is_number_and_color_possible(input: &str) -> IResult<&str, bool> {
    let (rest, (number_str, color)) = separated_pair(digit1, tag(" "), parse_color)(input)?;
    let number = u32::from_str_radix(number_str, 10).unwrap();
    if color == "red" && number <= 12 || color == "blue" && number <= 14 || color == "green" && number <= 13 {
        return Ok( (rest, true) );
    }
    Ok( (rest, false) )
}

fn is_game_possible(input: &str) -> IResult<&str, (u32, bool)> {
    let (mut rest, game_id_str) = delimited(tag("Game "), digit1, tag(": "))(input)?;
    let game_id = u32::from_str_radix(game_id_str, 10).unwrap();
    let mut is_dice_possible: bool;
    loop {
        (rest, is_dice_possible) = is_number_and_color_possible(rest)?;
        if is_dice_possible {
            match alt((tag::<&str, &str, Error<&str>>(", "), tag::<&str, &str, Error<&str>>("; ")))(rest) {
                Ok( (new_rest, _) ) => {
                    rest = new_rest;
                },
                Err(_) => {
                    return Ok( (rest, (game_id, true)) );
                }
            }
        } else {
            return Ok( (rest, (game_id, false)) );
        }
    }
}


fn main() {
    //println!("{:#?}", is_game_possible("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nblibla"));
    //println!("{:#?}", is_game_possible("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\nblub"));

    let mut possible_game_id_sum = 0u32;
    let stdin = io::stdin();

    for line in stdin.lines() {
        //println!("{:?}", line);
        //println!("{:#?}", is_game_possible(&line.unwrap()));
        let (_, (cur_game_id, is_cur_game_possible)) = is_game_possible(line.unwrap().as_str()).unwrap();
        if is_cur_game_possible {
            possible_game_id_sum += cur_game_id;
        }
    }

    println!("{}", possible_game_id_sum);

    /*
    fold_many0(
        is_game_possible,
        || { 0 },
        | mut acc: u32, (cur_game_id, is_cur_game_possible) | { if is_cur_game_possible { acc += cur_game_id } }
    )(
    */

}
