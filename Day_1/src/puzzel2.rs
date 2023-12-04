#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Lines;
    use super::*;

    #[test]
    fn it_works() {
        let input = "\
                            two1nine
                            eightwothree
                            abcone2threexyz
                            xtwone3four
                            4nineeightseven2
                            zoneight234
                            7pqrstsixteen";

        assert_eq!(String::from("281"), process_input(input).unwrap());
    }

    pub fn process_input(input: &str) -> io::Result<String>{

        let lines = input.lines();
        let finalnum = lines.map(|line| get_first_and_last(line)).sum::<i32>();

        Ok(finalnum.to_string())

    }



    pub fn get_first_and_last(line: &str) -> i32 {
        let mut temp_number = vec![];
        for (index, _c) in line.chars().enumerate(){
            let row = &line[index..];
            let result = if row.starts_with("one") || row.starts_with("1") {
                '1'
            } else if row.starts_with("two") {
                '2'
            } else if row.starts_with("three") {
                '3'
            } else if row.starts_with("four")  {
                '4'
            } else if row.starts_with("five") {
                '5'
            } else if row.starts_with("six") {
                '6'
            } else if row.starts_with("seven") {
                '7'
            } else if row.starts_with("eight") {
                '8'
            } else if row.starts_with("nine") {
                '9'
            } else if row.chars().next().unwrap().is_numeric() {
                row.chars().next().unwrap()
            } else {
                '0'
            };
            if(result != '0'){
                temp_number.push(result);
            }
        }
        let first = temp_number[0];
        let last = temp_number[temp_number.len() - 1];

        format!("{}{}", first, last).parse::<i32>().unwrap()
    }
}