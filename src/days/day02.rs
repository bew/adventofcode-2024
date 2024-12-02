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

// Copied from https://stackoverflow.com/a/79061120/5655255
fn skip_nth<I: Iterator>(iter: I, n: usize) -> impl Iterator<Item = I::Item> {
    iter.enumerate().filter_map(move |(idx, item)| {
        if idx == n {
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

    const EXAMPLE_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_parsing() {
        let parsed = parse_input(EXAMPLE_INPUT);
        dbg!(&parsed);
        assert_eq!(parsed, vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]);
    }

    #[test]
    fn test_example_part1() {
        let res = solve_part1(EXAMPLE_INPUT);
        assert_eq!(res, Some(2));
    }

    #[test]
    fn test_example_part2() {
        let res = solve_part2(EXAMPLE_INPUT);
        assert_eq!(res, Some(4));
    }

    #[test]
    fn test_report_safety_check() {
        let safe_report: Report = vec![7, 6, 4, 2, 1];
        assert!(report_is_safe(&safe_report));

        // `6 2`: delta is too large
        let unsafe_report1: Report = vec![9, 7, 6, 2, 1];
        assert!(!report_is_safe(&unsafe_report1));

        // `1 3` is increasing, then `3 2` is decreasing
        let unsafe_report2: Report = vec![1, 3, 2, 4];
        assert!(!report_is_safe(&unsafe_report2));
    }

    #[test]
    fn test_fixable_report() {
        // Can remove a `4` to make it safe
        let fixable_report: Report = vec![8, 6, 4, 4, 1];
        assert!(can_fix_report(&fixable_report));

        // Cannot be fixed, delta jump is too high
        let unfixable_report: Report = vec![1, 2, 7, 8, 9];
        assert!(!can_fix_report(&unfixable_report));
    }

    #[test]
    fn test_util_skip_nth() {
        let base = ["zero", "one", "two", "three"];
        let updated: Vec<_> = skip_nth(base.iter(), 2).cloned().collect();
        assert_eq!(updated, vec!["zero", "one", "three"]); // "two" has been skipped
    }
}
