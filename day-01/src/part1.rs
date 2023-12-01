pub fn process(input: &str) -> Result<i32, crate::error::Error> {
    Ok(input
        .lines()
        .map(|line| {
            let mut num: (Option<char>, Option<char>) = (None, None);
            line.chars().for_each(|char| {
                if char.is_numeric() {
                    if num.0.is_none() {
                        num.0 = Some(char);
                    } else {
                        num.1 = Some(char);
                    }
                }
            });

            let line_result: i32 = match num {
                (Some(char1), Some(char2)) => format!("{char1}{char2}").parse().unwrap(),
                (Some(char1), None) => format!("{char1}{char1}").parse().unwrap(),
                _ => 0,
            };

            line_result
        })
        .sum())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "#;

        assert_eq!(Ok(142), process(input))
    }
}
