use std::fs::set_permissions;
use nom::character::complete::newline;
use nom::multi::separated_list1;


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
    bit: i32,
    hand: Vec<&'a str>,
    points: Option<i32>,
}

impl<'a> Hand<'a> {

    fn finalpoint(&self) -> i32 {
        self.rank.unwrap() * self.bit
    }
    fn set_hand_and_bit(&mut self,bit: i32, hand: Vec<&'a str>){
        self.bit = bit;
        self.hand = hand;
    }


}

pub fn process_input(input: &str) -> i32{

    let (input, cards) = separated_list1(newline, hand)();

    0

}

pub fn hand(input: &str) -> Hand {
    let bit = input.split(" ").collect()[1]  as i32;
    let hand = input.split(" ").collect::<Vec<str>>()[0].split("").collect::<Vec<&str>>();
    Hand {
        rank: None,
        bit,
        hand,
        points: None,
    }
}
