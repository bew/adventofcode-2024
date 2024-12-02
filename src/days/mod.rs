mod day01;

pub type PartFn = fn(&str) -> Option<usize>;

pub struct Day {
    pub name: &'static str,
    pub description: &'static str,
    pub part1: Part,
    pub part2: Part,
    pub default_input: &'static str,
}

pub struct Part {
    pub func: PartFn,
    pub expected: Option<usize>,
}

macro_rules! def_day {
    ($d: ident, desc=$desc: tt, expected=($exp1: expr, $exp2: expr)) => {
        Day {
            name: stringify!($d),
            description: stringify!($desc),
            part1: Part {
                func: $d::solve_part1,
                expected: $exp1,
            },
            part2: Part {
                func: $d::solve_part2,
                expected: $exp2,
            },
            default_input: include_str!(concat!("../../inputs/", stringify!($d), ".txt")),
        }
    };
}

pub static DAYS: &[Day] = &[
    def_day!(day01, desc="Historian Hysteria", expected=(None, None)),
];
