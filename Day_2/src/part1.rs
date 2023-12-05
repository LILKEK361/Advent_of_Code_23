use std::collections::{BTreeMap, HashMap};
use std::io;
use nom::bytes::streaming::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, digit1, newline};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(8,process_input(input).unwrap());
    }


}

pub struct Cube<'a> {
    val:   u32,
    color: &'a str,
}
pub struct Game<'a> {
    game_id: &'a str,
    sets: Vec<Vec<Cube<'a>>>
}

impl<'a> Game<'a> {
    fn valid(&self, colors: &BTreeMap<&str, i32>) -> Option<i32> {
        self.sets.iter().all(|round| {
            round.iter().all(|shown_cube| {
                shown_cube.val <= *colors.get(shown_cube.color).expect("a valid cube")
            })
        }).then_some(
            self.game_id.parse::<u32>().expect("This should parse")
        )
    }
}


fn game(input: &str) -> IResult<&str, Vec<Game>> {
    let (input , games) = separated_list1(newline, seperate_game)(input)?;
    Ok((input, games))
}
fn seperate_game(game: &str) -> IResult<&str, Game>{
    let (game, game_id ) = preceded(tag("Game "), digit1)(game)?;
    let (game, rounds) =  preceded(tag(": "), separated_list1(tag(";"), round))(game)?;
    Ok((game, Game { game_id, sets: rounds, }))
}


pub fn round(input: &str) -> IResult<&str,Vec<Cube>>{
    let (game, cubes) = separated_list1(tag(", " ),cube)(input)?;
    Ok((game, cubes ))
}
pub fn cube(input: &str) -> IResult<&str, Cube>{
    let(input, (value, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((input, Cube {val: value, color}))
}
#[tracing::instrument]
pub fn process_input(input: &str) -> io::Result<i32> {



    let map = BTreeMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let games = game(input).unwrap();
    Ok(games.1.iter().filter_map(|game| game.valid(&map)).sum::<i32>())


}

