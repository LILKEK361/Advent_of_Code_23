use std::fs::set_permissions;
use nom::bytes::complete::tag;
use nom::character::complete::{digit0, line_ending, newline};
use nom::character::is_alphabetic;
use nom::IResult;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::preceded;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {

        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let final_n = process_input(input).to_string();

        assert_eq!("6440",final_n);
    }
}

enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Bub = 11,
    Queen = 12,
    King = 13,
    Ass = 14,
}
enum HandTypes{
    Five
}

/*
Five of a kind
Four of a kind
Full House
Three of a kind
Two of a kind
Highest card
 */
pub struct Hand<'a> {
    rank: Option<i32>,
    bit: &'a str,
    hand: Vec<&'a char>,
    points: Option<i32>,
}

impl<'a> Hand<'a> {

    fn worthcheck(&self) -> i32 {

    }
}

pub fn process_input(input: &str) -> i32{
    /*
    Plan:
    Create vec of cards
    Check if the hands are equal worth
               Check for char in Hand

    Push worth_array
    for i in 0..wortharray_len()
        finalnum += wortharray_len[i] * i + 1;

    format!("", finalnum)


    */
    let (input, hands) = separated_list0(line_ending, hand_parser)(input).unwrap();
    hands;
    0

}

pub fn hand_parser(input: &str) -> IResult<&str, Hand> {
    let input_split = input.split_whitespace().collect::<Vec<&str>>();
    let bit = input_split[1];
    let  hand = input_split[0].chars().collect();

    Ok((input, Hand {
        rank: None,
        bit,
        hand,
        points: None,
    }))

}

