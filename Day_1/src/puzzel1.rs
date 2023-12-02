
#[tracing::instrument]
pub fn process(input: &str) -> io::Result<()>{

    let result = input.lines()
            .map(|singles| {
                let num = singles.chars()
                    .filter_map(|char| {
                        char.is_digit(10);
                    });

                let first_number = num.next().expect("This should be a number ");

                match it.last() {
                    Some(num) => format!("{first_num}{num}"),
                    None => format!("{first_num}{first_num}"),
                }
                    .parse::<i32>()
                    .expect("This should be a valid number ")
            }).sum::<i32>();
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "\
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        assert_eq!("142",process(inputs));
    }

}
