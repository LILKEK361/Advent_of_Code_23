




mod part1{
    use std::collections::HashMap;

    fn process_input(input: &str, blue: i32, red: i32, green: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    let games = input.lines();

    for mut single in games {

        let mut line_string:Vec<&str> = single.split_whitespace().collect();

        let game_id = line_string[1].replace(":","");




        let mut sets: Vec<Vec<String>>= vec![];
        let mut temp: Vec<String> = vec![];
        for i in 2..line_string.len() {

            if(!line_string[i].contains(";") ) {
                temp.push(line_string[i].parse::<String>().unwrap())
            } else if(i == line_string.len()){
                temp.push(line_string[i].parse::<String>().unwrap());
                sets.push(temp.clone());
            }else {
                temp.push(line_string[i].parse::<String>().unwrap());
                sets.push(temp.clone());
                temp = vec![];
            }
        }
        let mut checkmark = 0;
        for  set in sets.iter() {
            let mut points: HashMap<&str, i32> = HashMap::from([
                ("red", 0),
                ("green", 0),
                ("blue", 0),

            ]);

            for i in 0..set.len(){
                for (key, value) in points.clone().iter() {
                    if(set[i].contains(key)){
                        let newnum = set[(i - 1)].parse::<i32>().unwrap();
                        let oldnum = points[key];
                        let fin = oldnum + newnum;
                        *points.entry(key).or_insert(0) += fin;

                    }
                }


            }
            if(points["green"] <= green && points["red"] <= red && points["blue"] <= blue){
                println!("{:?}", set);
                checkmark += 1

            }
        }

        if(checkmark ==  sets.len()){
            println!("game is valid: {:?}", game_id);
            result.push(game_id.parse().unwrap())
        }

    }

    result
}

    pub fn get_str_to_i32(input: &str) -> i32 {

        input.parse::<i32>().unwrap()

    }
}