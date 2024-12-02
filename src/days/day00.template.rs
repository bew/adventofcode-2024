use nom::{
    character::{self as chr, complete::newline},
    combinator::map,
    multi::separated_list1,
    IResult
};

fn parse_input(input: &str) -> Vec<usize> {
    let line_parser = map(chr::complete::u64, |i| i as usize);
    let res: IResult<&str, Vec<usize>> = separated_list1(newline, line_parser)(input);
    let (_, parsed) = res.unwrap();
    parsed
}

pub fn solve_part1(input: &str) -> Option<usize> {
    let _parsed = parse_input(input);

    // Do something with parsed input

    None
}

pub fn solve_part2(input: &str) -> Option<usize> {
    let _parsed = parse_input(input);

    // Do something with parsed input

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_parsing() {
        let input = "123\n456\n789";
        let parsed = parse_input(input);
        dbg!(&parsed);
        assert_eq!(parsed, vec![123_usize, 456_usize, 789_usize]);
    }
}
