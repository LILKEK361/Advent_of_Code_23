use std::fs::set_permissions;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::IResult;
use nom::multi::separated_list1;
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
    hand: Vec<&'a str>,
    points: Option<i32>,
}

impl<'a> Hand<'a> {


    fn set_hand_and_bit(&mut self,bit: &'a str, hand: Vec<&'a str>){
        self.bit = bit;
        self.hand = hand;
    }


}

pub fn process_input(input: &str) -> i32{

    let (input, hands) = separated_list1(newline, hand)(input).unwrap();

    0

}

pub fn hand(input: &str) -> IResult<&str, Hand> {
    let bit = input.split(" ").collect::<Vec<&str>>()[1];
    let (input, hand) = preceded(tag(""), tag(" "))(input)?;

    Ok((input, Hand {
        rank: None,
        bit,
        hand    ,
        points: None,
    }))
}
