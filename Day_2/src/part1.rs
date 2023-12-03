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

pub fn color_check(key: &str, vaule: &i32, blue: i32, red: i32, green: i32) -> bool {
    match key{
        "blue" => if(vaule > &blue ){
            false
        }else { true}
        "red" => if(vaule > &red ){
            false
        }else { true}
        "green" => if(vaule > &green ){
            false
        }else { true},
        _ => {false}
    }
}
#[tracing::instrument]
pub fn process_input(input: &str, blue: i32, red: i32, green: i32) -> i32 {
    let mut result: Vec<i32> = vec![];

    let games = input.lines();

    for mut single in games {
        let mut line_string: Vec<&str> = single.split_whitespace().collect();
        let mut overall: i32 = 0;
        let game_id = line_string[1].replace(":", "");

        let color = ["red","green", "blue"];
        let mut sets: Vec<Vec<String>> = vec![];
        let mut temp: Vec<String> = vec![];

        for i in 2..line_string.len() {
            if (!line_string[i].contains(";")) {
                temp.push(line_string[i].parse::<String>().unwrap())
            } else if (i == line_string.len()) {
                temp.push(line_string[i].parse::<String>().unwrap());
                sets.push(temp.clone());
            } else {
                temp.push(line_string[i].parse::<String>().unwrap());
                sets.push(temp.clone());
                temp = vec![];
            }
        }
        for set in sets.iter() {
            let mut points: HashMap<&str, i32> = HashMap::from([
                ("red", 0),
                ("green", 0),
                ("blue", 0),
            ]);

            for i in 0..set.len() {
                for (key, value) in points.clone().iter() {
                    if (set[i].contains(key)) {
                        let newnum = set[(i - 1)].parse::<i32>().unwrap();
                        let oldnum = value;
                        let fin = oldnum + newnum;
                        if(color_check(key, &fin, blue, red, green)){
                            *points.entry(key).or_insert(0) += fin;
                        }else {

                            break
                        }


                    }

                }

            }
            let mut ch: i32 = 0;
            for (key, value) in points.iter()  {
                if(color_check(key, value, blue, red, green)){
                        ch += 1;
                }else {
                    println!("Not valid");
                    break
                }
            }
            if(ch == 3){
                overall += 1
            }

            if(overall == sets.len().try_into().unwrap()){
                result.push(game_id.parse::<i32>().unwrap());
            }

        }


    }
    let mut numb = 0;
    for i in result {
        numb += i;
    }

    numb

}

pub fn get_str_to_i32(input: &str) -> i32 {
    input.parse::<i32>().unwrap()
}