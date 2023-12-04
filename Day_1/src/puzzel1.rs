

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Lines;
    use super::*;
    #[test]
    fn it_works() {
        let input = "\
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        assert_eq!(String::from("142"),process(input));
    }
    #[tracing::instrument]
    pub fn process(input: &str) -> String{

        let result = input.lines();
        let mut finali32 = 0;
        for i in result {
            let mut finalresult: String = String::new();
            finalresult.push(get_first_num(i));
            finalresult.push(get_first_num((i.chars().rev().collect::<String>()).as_str()));
            finali32 += finalresult.parse::<i32>().unwrap();
        }

        finali32.to_string()
    }
    pub fn get_first_num(stri: &str) -> char {
        let mut first: char = '0';
        for i in stri.chars(){
           if(i.is_ascii_digit()) {
               first = i;
               break;
           }
        }
        first
    }
}
