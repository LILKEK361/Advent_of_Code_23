use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, newline};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::preceded;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = " Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!("13", process_input(input).unwrap().to_string());
    }
}

pub struct Card<'a> {
    winningnumbers: Vec<&'a str>,
    points: i32,
    numbers: Vec<&'a str>,
}

impl<'a> Card<'a> {
    fn valid(&mut self) -> i32 {
        for number in &self.numbers {
            if self.winningnumbers.contains(&number) {
                if self.points == 0i32 {
                    self.points = 1;
                } else {
                    self.points = self.points * 2
                }
            }
        }
        self.points
    }
}


pub fn process_input(input: &str) -> Result<i32, io::Error> {
    let cards = card_parser(input) ;
    let mut final_num = 0;
    for mut card in cards {
        final_num += card.valid();
    }

    Ok(final_num)
}

pub fn card_parser(input: &str) -> Vec<Card>  {
    let mut cards = vec![];
    for mut line in input.lines() {
        let mut card = line.split_whitespace().collect::<Vec<&str>>();
        let mut spit = false;
        let mut winning = vec![];
        let mut numbers = vec![];
        for i in 2..card.len()   {

            if !card[i].eq("|") && !spit {
                winning.push(card[i])
            }else if card[i].eq("|") {
                spit = true;
            }else{
                numbers.push(card[i])
            }
        }
        cards.push(Card {
            winningnumbers: winning,
            points: 0,
            numbers,
        })
    }
 cards
}

