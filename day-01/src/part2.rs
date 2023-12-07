pub fn process(input: &str) -> Result<i32, crate::error::Error> {
    Ok(input.lines().map(calc_line).sum())
}

pub fn calc_line(line: &str) -> i32 {
    let mut num: (Option<i32>, Option<i32>) = (None, None);

    if line.is_empty() {
        return 0;
    };

    for window_end in 1..line.len() {
        let first_val = match &line[0..window_end] {
            one if one.contains("one") || one.contains('1') => 1,
            two if two.contains("two") || two.contains('2') => 2,
            three if three.contains("three") || three.contains('3') => 3,
            four if four.contains("four") || four.contains('4') => 4,
            five if five.contains("five") || five.contains('5') => 5,
            six if six.contains("six") || six.contains('6') => 6,
            seven if seven.contains("seven") || seven.contains('7') => 7,
            eight if eight.contains("eight") || eight.contains('8') => 8,
            nine if nine.contains("nine") || nine.contains('9') => 9,
            _ => 0,
        };

        if first_val != 0 {
            num.0 = Some(first_val);
            break;
        }
    }

    for window_start in 1..line.len() {
        let window_start = line.len() - window_start;
        let last_val = match &line[window_start..line.len()] {
            one if one.contains("one") || one.contains('1') => 1,
            two if two.contains("two") || two.contains('2') => 2,
            three if three.contains("three") || three.contains('3') => 3,
            four if four.contains("four") || four.contains('4') => 4,
            five if five.contains("five") || five.contains('5') => 5,
            six if six.contains("six") || six.contains('6') => 6,
            seven if seven.contains("seven") || seven.contains('7') => 7,
            eight if eight.contains("eight") || eight.contains('8') => 8,
            nine if nine.contains("nine") || nine.contains('9') => 9,
            _ => 0,
        };

        if last_val != 0 {
            num.1 = Some(last_val);
            break;
        }
    }

    match num {
        (Some(first), Some(last)) => format!("{first}{last}").parse().unwrap(),
        (Some(first), None) => format!("{first}{first}").parse().unwrap(),
        (None, Some(last)) => format!("{last}{last}").parse().unwrap(),
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "#;

        assert_eq!(Ok(281), process(input))
    }

    // TODO: add calc_line test
}
