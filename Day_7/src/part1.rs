use std::collections::HashMap;

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
        process_input(input);
        let highest_hand = "";


        assert_eq!("2oK", highest_hand);
    }
}


enum HandTypes {
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
    hand: Vec<&'a str>,
    hand_type: &'a str,
    points: Option<i32>,
}

pub fn hand_parser(input: &str) -> Hand {
    Hand::from_str(input)
}
impl<'a> Hand<'a> {
    fn from_str(s: &str) -> Hand {
        let input_split = s.split_whitespace().collect::<Vec<&str>>();

        let bit = input_split[1];
        let hand = input_split[0].split("").collect::<Vec<&str>>();

        Hand {
            rank: None,
            bit,
            hand,
            hand_type: Hand::get_hand(input_split[0]),
            points: None,
        }
    }

    fn get_hand(hand: &str) -> &str {
        let mut hand_type = "";
        let mut map: HashMap<String, i32> = HashMap::new();
        //Looping trough the hand to see how often a card is in the hand
        for card in hand.chars() {

            &map.entry(String::from(card.clone())).and_modify(|e| *e += 1).or_insert_with_key(|card| 1);

        }
        //Sorting the Cards so the most often card is first
        let mut hash_vec: Vec<(&String, &i32)> = map.iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(a.1));

        //Decide witch Hand_type a player has
        match hash_vec[0].1 {
            5 => hand_type = "5ok",
            4 => hand_type = "4ok",
            3 => {
                if (hash_vec.len() > 2) {
                    hand_type = "3oK";
                } else if (hash_vec.len() == 2) {
                    if (*hash_vec[1].1 as i32 == 2) {
                        hand_type = "FH";
                    }
                }
            }
            2 => hand_type = "2oK",
            1 => hand_type = "HC",

            &_ => {}
        }
        hand_type
    }
}
//Rework
pub fn sort_hand_by_worth(mut hands: Vec<Hand>) -> Vec<Hand>{
    let order = vec!["HC", "2oK", "3oK", "4oK", "5oK"];
    let mut swapped;
    loop {

        swapped = false;

        for i in 0..(hands.len() - 1) {
            if order.iter().find(hands[i].hand_type).unwrap() > order.iter().find(hands[i + 1].hand_type).unwrap(){

                hands.swap(i, i + 1);
                swapped = true;

            } else if order.iter().find(hands[i].hand_type).unwrap() == order.iter().find(hands[i + 1].hand_type).unwrap() {

            }
        }

        if !swapped {
            break;
        }
    }

    hands

}

pub fn process_input(input: &str) {
    let Hands = input.lines().map(|line| Hand::from_str(line)).collect::<Vec<Hand>>();
    let sorted_hand = sort_hand_by_worth(Hands);
}


