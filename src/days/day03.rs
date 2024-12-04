use nom::{
    branch::alt, bytes::complete::{tag, take}, character::complete::{char, u32 as parse_u32}, combinator::{map, not}, multi::{many0_count, many1}, sequence::{delimited, preceded, separated_pair}, IResult
};

#[derive(Debug, PartialEq, Eq)]
enum Ast {
    MulNode(u32, u32),
    Do,
    Dont,
}

// Format: `mul(123,42)`
fn parse_mul(input: &str) -> IResult<&str, Ast> {
    let (input, _) = tag("mul")(input)?;
    let (input, (n1, n2)) = delimited(
        char('('),
        separated_pair(parse_u32, tag(","), parse_u32),
        char(')'),
    )(input)?;
    Ok((input, Ast::MulNode(n1, n2)))
}

fn parse_char_not_mul(input: &str) -> IResult<&str, &str> {
    // NOTE: `not(parse_mul)` succeeds when not on a mul node, but does NOT comsume any input,
    //   so we need to manually take it ourselves.
    // NOTE: `preceded(A, B)` discards A's output and keeps B's.
    preceded(not(parse_mul), take(1_usize))(input)
}

fn parse_input_with_muls(input: &str) -> IResult<&str, Vec<Ast>> {
    // NOTE: using many0_count instead of many0 to avoid collecting chars (will be discarded anyway)
    let parse_any_gibberish = many0_count(parse_char_not_mul);
    let (rest, mul_nodes) = many1(preceded(parse_any_gibberish, parse_mul))(input)?;
    Ok((rest, mul_nodes))
}

pub fn solve_part1(input: &str) -> Option<usize> {
    let (_, nodes) = parse_input_with_muls(input).unwrap();

    let result = nodes.iter()
        .map(|ast_node| {
            let Ast::MulNode(n1, n2) = ast_node else { unreachable!("part1 only has MulNode"); };
            (*n1 as usize) * (*n2 as usize)
        })
        .sum();

    Some(result)
}

// ----------------------------------------------------

fn parse_do(input: &str) -> IResult<&str, Ast> {
    map(tag("do()"), |_| Ast::Do)(input)
}

fn parse_dont(input: &str) -> IResult<&str, Ast> {
    map(tag("don't()"), |_| Ast::Dont)(input)
}

fn parse_ast(input: &str) -> IResult<&str, Ast> {
    alt((
        parse_mul,
        parse_dont, // NOTE: must be before `do` to work
        parse_do,
    ))(input)
}

fn parse_char_not_ast(input: &str) -> IResult<&str, &str> {
    // REF: see `parse_char_not_mul`
    preceded(not(parse_ast), take(1_usize))(input)
}

fn parse_input_with_ast(input: &str) -> IResult<&str, Vec<Ast>> {
    // NOTE: using many0_count instead of many0 to avoid collecting chars (will be discarded anyway)
    let parse_any_gibberish = many0_count(parse_char_not_ast);
    let (rest, ast_nodes) = many1(preceded(parse_any_gibberish, parse_ast))(input)?;
    Ok((rest, ast_nodes))
}

pub fn solve_part2(input: &str) -> Option<usize> {
    // Parse input
    let (_, ast_nodes) = parse_input_with_ast(input).unwrap();

    let mut actions_enabled = true;
    let result = ast_nodes.into_iter()
        .filter_map(|ast_node| {
            match ast_node {
                Ast::MulNode(n1, n2) => {
                    if actions_enabled {
                        Some((n1 as usize) * (n2 as usize))
                    } else {
                        None
                    }
                },
                Ast::Dont => {
                    actions_enabled = false;
                    None
                },
                Ast::Do => {
                    actions_enabled = true;
                    None
                },
            }
        })
        .sum();

    Some(result)
}

// ----------------------------------------------------

#[cfg(test)]
mod tests_part1 {
    use nom::multi::many0;

    use super::*;
    use super::Ast::MulNode;

    const EXAMPLE_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_parse_mul_node() {
        let input = "mul(123,42)bla";
        let res = parse_mul(input);
        dbg!(&res);
        let (rest, node) = res.unwrap();
        assert_eq!(rest, "bla");
        assert_eq!(node, MulNode(123, 42));
    }

    #[test]
    fn test_parse_gibberish_until_mul() {
        let input = "xyzmul(1,2)bla";
        let res = many0(parse_char_not_mul)(input);
        dbg!(&res);
        let (rest, gibberish) = res.unwrap();
        assert_eq!(rest, "mul(1,2)bla");
        assert_eq!(gibberish, vec!["x", "y", "z"]);
    }

    // NOTE: missing gibberish must be ok to work as separator between good mul nodes
    #[test]
    fn test_parse_missing_gibberish_as_ok() {
        let input = "mul(41,42)bla"; // zero gibberish before mul node
        let res = many0(parse_char_not_mul)(input);
        dbg!(&res);
        let (rest, gibberish) = res.unwrap();
        assert_eq!(rest, "mul(41,42)bla");
        assert_eq!(gibberish.len(), 0);
    }

    #[test]
    fn test_parse_chained_muls() {
        let input = "mul(1,2)mul(3,4)bla";
        // mul nodes:^------^^------^
        let res = parse_input_with_muls(input);
        dbg!(&res);
        let (rest, parsed) = res.unwrap();
        assert_eq!(rest, "bla");
        assert_eq!(parsed, vec![MulNode(1, 2), MulNode(3, 4)]);
    }

    #[test]
    fn test_parse_separated_muls() {
        let input = "xmul(2,4)%&mul[3,7]!do_not_mul(512,74)+mul(32,64]t";
        // mul nodes: ^------^                  ^---------^
        let res = parse_input_with_muls(input);
        dbg!(&res);
        let (rest, parsed) = res.unwrap();
        assert_eq!(rest, "+mul(32,64]t");
        assert_eq!(parsed, vec![MulNode(2, 4), MulNode(512, 74)]);
    }

    #[test]
    fn test_part1() {
        let res = solve_part1(EXAMPLE_INPUT);
        assert_eq!(res, Some(161));
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    const EXAMPLE_INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    // ast nodes:                 ^------^           ^-----^   (mul nodes disabled here)    ^--^ ^------^
    // effect:                                      (disables)                           (enables)

    #[test]
    fn test_parse_do_node() {
        let input = "do()bla";
        let res = parse_do(input);
        dbg!(&res);
        let (rest, node) = res.unwrap();
        assert_eq!(rest, "bla");
        assert_eq!(node, Ast::Do);
    }

    #[test]
    fn test_parse_dont_node() {
        let input = "don't()bla";
        let res = parse_dont(input);
        dbg!(&res);
        let (rest, node) = res.unwrap();
        assert_eq!(rest, "bla");
        assert_eq!(node, Ast::Dont);
    }

    #[test]
    fn test_parse_input_with_ast() {
        let input = "xmul(2,4)&mul[3do(),7]!^dont()!mul(41,42)_undon't()bla)";
        // ast nodes: ^------^      ^-----^         ^--------^     ^--^
        let res = parse_input_with_ast(input);
        dbg!(&res);
        let (rest, parsed) = res.unwrap();
        assert_eq!(rest, "bla)");
        assert_eq!(parsed, vec![
            Ast::MulNode(2, 4),
            Ast::Do,
            Ast::MulNode(41, 42),
            Ast::Dont,
        ]);
    }

    #[test]
    fn test_part2() {
        let res = solve_part2(EXAMPLE_INPUT);
        assert_eq!(res, Some(48));
    }
}
