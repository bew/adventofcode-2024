use nom::{
    bytes::complete::tag, character::{self as chr, complete::newline}, multi::separated_list1, sequence::separated_pair, IResult
};
use sorted_vec::SortedVec;

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let line_parser = separated_pair(chr::complete::u64, tag("   "), chr::complete::u64);
    let res: IResult<&str, Vec<(u64, u64)>> = separated_list1(newline, line_parser)(input);
    let (_, parsed) = res.unwrap();
    parsed
}

pub fn solve_part1(input: &str) -> Option<usize> {
    let both_lists = parse_input(input);

    // Separate in 2 sorted lists
    let mut left_list: SortedVec<u64> = SortedVec::new();
    let mut right_list: SortedVec<u64> = SortedVec::new();
    for (left, right) in both_lists.iter() {
        left_list.push(*left);
        right_list.push(*right);
    }

    // Now, go through both lists, and sum all their delta
    let result: u64 = left_list.into_vec()
        .iter()
        .zip(right_list.into_vec().iter())
        .map(|(&left, &right)| left.max(right).saturating_sub(left.min(right)))
        .sum();

    Some(result as usize)
}

pub fn solve_part2(_input: &str) -> Option<usize> {
    None
}
