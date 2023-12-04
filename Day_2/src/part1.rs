use std::collections::HashMap;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "\
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(8,process_input(input,14,13,12));
    }


}
fn count_char(s: &str, c: char) -> usize {
    s.chars().filter(|&x| x == c).count()
}

#[tracing::instrument]
pub fn process_input(input: &str, blue: i32, red: i32, green: i32) -> i32 {

    let mut lines = input.lines();
    let mut finalgames = vec![];

    for line in lines {

        let mut blueCubes = 0;
        let mut redCubes = 0;
        let mut greencubes = 0;

        let game = line.split_whitespace().collect::<Vec<&str>>();

        let game_id = game[1];

        let sets = (count_char(line, ';') + 1) as i32 ;



        for i in 2..game.len() {
            if(game[i].eq( "green")){
                greencubes += (&game[i - 1]).parse::<i32>().unwrap()
            }
            if(game[i].eq("blue")){
                blueCubes += (&game[i - 1]).parse::<i32>().unwrap()
            }
            if(game[i].eq("red")){
                redCubes += (&game[i - 1]).parse::<i32>().unwrap()
            }
        }

    }
    let mut finalnumber = 0;
    for i in finalgames{
        finalnumber += i.parse::<i32>().unwrap();
    }

    finalnumber

}

