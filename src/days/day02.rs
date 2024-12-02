use nom::{bytes::complete::tag, character::{self as chr, complete::newline}, multi::separated_list1, IResult};

type Report = Vec<i32>;

fn parse_input(input: &str) -> Vec<Report> {
    let report_parser = separated_list1(tag(" "), chr::complete::i32);
    let res: IResult<&str, Vec<Report>> = separated_list1(newline, report_parser)(input);
    let (_, parsed) = res.unwrap();
    parsed
}

fn report_is_safe(report: &Report) -> bool {
    // Report safety rules:
    // - The levels are either all increasing or all decreasing.
    // - Any two adjacent levels differ by at least one and at most three.

    let deltas: Vec<i32> = report.windows(2).map(|win| win[1] - win[0]).collect();

    let first_delta_sign = deltas[0].signum();
    let all_same_sign = deltas.iter().all(|d| d.signum() == first_delta_sign);

    let delta_within_acceptable_range = deltas.iter().all(|d| {
        let abs = d.abs();
        1 <= abs && abs <= 3
    });

    // println!("For report: {report:?} (deltas: {deltas:?}) :: same sign? {all_same_sign} in_range? {delta_within_acceptable_range}");

    all_same_sign && delta_within_acceptable_range
}

pub fn solve_part1(input: &str) -> Option<usize> {
    let reports = parse_input(input);

    let num_safe_reports = reports.iter()
        .filter(|r| report_is_safe(r))
        .count();

    Some(num_safe_reports)
}

// --------------------------------------------------------

fn skip_nth<I: Iterator>(iter: I, n: usize) -> impl Iterator<Item = I::Item> {
    iter.enumerate().filter_map(move |(i, item)| {
        if i == n {
            None // Skip the N-th element
        } else {
            Some(item) // Keep all other elements
        }
    })
}

fn can_fix_report(report: &Report) -> bool {
    for skip_idx in 0..report.len() {
        let maybe_fixed_report: Report = skip_nth(report.iter(), skip_idx).cloned().collect();
        if report_is_safe(&maybe_fixed_report) {
            return true
        }
    }
    false
}

pub fn solve_part2(input: &str) -> Option<usize> {
    let reports = parse_input(input);

    let num_safe_reports = reports.iter()
        .filter(|r| report_is_safe(r))
        .count();
    let num_fixed_reports = reports.iter()
        .filter(|r| !report_is_safe(r))
        .filter(|r| can_fix_report(r))
        .count();

    Some(num_safe_reports + num_fixed_reports)
}

// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = "\
7 6 4 2 1
1 2 7 8 9
";
        let parsed = parse_input(input);
        dbg!(&parsed);
        assert_eq!(parsed, vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
        ]);
    }
}
