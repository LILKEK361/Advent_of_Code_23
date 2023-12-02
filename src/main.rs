use std::collections::HashMap;
use std::io;

fn main() {
       let test_input = "\
      Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        process_input(test_input,14 ,12,13 )
}

fn process_input(input: &str, blue: i32, red: i32, green: i32){
    let  result: HashMap<String, i32> = HashMap::new();

    let games = input.lines();

    for line in games {

    }
    println!("{:?}", games)



}