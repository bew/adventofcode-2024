use std::collections::HashMap;

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
    let mut left_list: SortedVec<usize> = SortedVec::new();
    let mut right_list: SortedVec<usize> = SortedVec::new();
    for (left, right) in both_lists.iter() {
        left_list.push(*left as usize);
        right_list.push(*right as usize);
    }

    // Now, go through both lists, and sum all their delta
    let result: usize = left_list.into_vec()
        .iter()
        .zip(right_list.into_vec().iter())
        .map(|(&left, &right)| left.max(right).saturating_sub(left.min(right)))
        .sum();

    Some(result)
}

pub fn solve_part2(input: &str) -> Option<usize> {
    let both_lists = parse_input(input);

    // Separate in 2 lists
    let mut left_list: Vec<usize> = Vec::new();
    let mut right_list: Vec<usize> = Vec::new();
    for (left, right) in both_lists.iter() {
        left_list.push(*left as usize);
        right_list.push(*right as usize);
    }

    // Count each item in right list
    let mut right_ids_counter = HashMap::new();
    for right_item in right_list.iter() {
        if let Some(_) = right_ids_counter.get(right_item) {
            continue // We already have this id's count, skip
        }
        let item_count = right_list.iter().filter(|&it| it == right_item).count();
        right_ids_counter.insert(*right_item, item_count);
    }

    // Compute similarity score based on left list
    let similarity_score = left_list.iter()
        .map(|it| it * right_ids_counter.get(it).unwrap_or(&0))
        .sum();

    Some(similarity_score)
}
