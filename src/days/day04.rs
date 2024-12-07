#[derive(PartialEq, Eq, Clone, Copy)]
struct Pos {
    pub x: i16,
    pub y: i16,
}
impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos(x: {x}, y: {y})", x = self.x, y = self.y)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
}
impl Grid {
    fn new(data: Vec<Vec<char>>) -> Self {
        // note: we know all lines have same length, so can get width from first line only
        let width = {
            let first_line = data.iter().next().unwrap();
            first_line.len()
        };
        let height = data.len();
        Self { data, width, height }
    }

    fn get_at(&self, pos: &Pos) -> Option<char> {
        if pos.x < 0 || pos.y < 0 || self.width <= (pos.x as usize) || self.height <= (pos.y as usize) {
            return None;
        }
        let line = self.data.get(pos.y as usize);
        let maybe_char = line.and_then(|line| line.get(pos.x as usize));
        maybe_char.copied()
    }

    fn iter_with_position(&self) -> impl Iterator<Item = (Pos, char)> + use<'_> {
        self.data.iter()
            .enumerate()
            .flat_map(|(y_idx, line)| {
                line.iter()
                    .copied()
                    .enumerate()
                    .map(move |(x_idx, chr)| {
                        let pos = Pos {
                            x: x_idx as i16,
                            y: y_idx as i16,
                        };
                        (pos, chr)
                    })
            })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}
impl Direction {
    fn next_pos(&self, pos: &Pos) -> Pos {
        match self {
            Direction::TopLeft => {
                Pos { x: pos.x - 1, y: pos.y - 1 }
            },
            Direction::Top => {
                Pos { x: pos.x,     y: pos.y - 1 }
            },
            Direction::TopRight => {
                Pos { x: pos.x + 1, y: pos.y - 1 }
            },
            Direction::Left => {
                Pos { x: pos.x - 1, y: pos.y }
            },
            Direction::Right => {
                Pos { x: pos.x + 1, y: pos.y }
            },
            Direction::BottomLeft => {
                Pos { x: pos.x - 1, y: pos.y + 1 }
            },
            Direction::Bottom => {
                Pos { x: pos.x,     y: pos.y + 1 }
            },
            Direction::BottomRight => {
                Pos { x: pos.x + 1, y: pos.y + 1 }
            },
        }
    }

    fn reverse_dir(&self) -> Self {
        match self {
            Direction::TopLeft => Direction::BottomRight,
            Direction::Top => Direction::Bottom,
            Direction::TopRight => Direction::BottomLeft,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::BottomLeft => Direction::TopRight,
            Direction::Bottom => Direction::Top,
            Direction::BottomRight => Direction::TopLeft,
        }
    }
}

fn parse_input(input: &str) -> Grid {
    let parsed = input.lines().map(|line| line.chars().collect()).collect();
    Grid::new(parsed)
}

#[derive(Debug)]
struct WordSearch {
    char_positions: Vec<(Pos, char)>,
    #[allow(dead_code)] // needed because never read, but useful for debugs
    dir: Direction,
}
impl WordSearch {
    fn from_word_and_vector(word: &str, start_pos: &Pos, dir: &Direction) -> Self {
        let mut next_pos = *start_pos;
        let mut char_positions = vec![];
        for chr in word.chars() {
            char_positions.push((next_pos, chr));
            next_pos = dir.next_pos(&next_pos);
        }
        Self { char_positions, dir: *dir }
    }
}

fn is_word_matching(grid: &Grid, word_search: &WordSearch) -> bool {
    word_search.char_positions
        .iter()
        .all(|&(pos, expected_chr)| {
            let Some(chr) = grid.get_at(&pos) else {
                // No char at that pos, no match
                return false;
            };
            chr == expected_chr
        })
}

pub fn solve_part1(input: &str) -> Option<usize> {
    const WORD_SEARCH: &str = "XMAS";

    let grid = parse_input(input);

    let all_x_iter = grid.iter_with_position()
        .filter(|&(_, chr)| chr == 'X');

    // For all starting positions of the X for XMAS, search for the word in all directions
    let mut num_xmas_matches = 0;
    for (pos, _x_chr) in all_x_iter {
        let searches = [
            WordSearch::from_word_and_vector(WORD_SEARCH, &pos, &Direction::TopLeft),
            WordSearch::from_word_and_vector(WORD_SEARCH, &pos, &Direction::Top),
            WordSearch::from_word_and_vector(WORD_SEARCH, &pos, &Direction::TopRight),
            WordSearch::from_word_and_vector(WORD_SEARCH, &pos, &Direction::Left),
            WordSearch::from_word_and_vector(WORD_SEARCH, &pos, &Direction::Right),
            WordSearch::from_word_and_vector(WORD_SEARCH, &pos, &Direction::BottomLeft),
            WordSearch::from_word_and_vector(WORD_SEARCH, &pos, &Direction::Bottom),
            WordSearch::from_word_and_vector(WORD_SEARCH, &pos, &Direction::BottomRight),
        ];

        num_xmas_matches += searches.iter()
            .filter(|word_search| is_word_matching(&grid, word_search))
            .count()
    }

    Some(num_xmas_matches)
}

// ----------------------------------------------------

fn x_shape_word_searches_around_pos(word: &str, pos: &Pos) -> [WordSearch; 4] {
    let diag_word_search = |dir: &Direction| {
        // To get diagonal char positions, we go back 1 then go forward in dir
        WordSearch::from_word_and_vector(word, &dir.reverse_dir().next_pos(pos), &dir)
    };
    [
        diag_word_search(&Direction::TopLeft),
        diag_word_search(&Direction::TopRight),
        diag_word_search(&Direction::BottomRight),
        diag_word_search(&Direction::BottomLeft),
    ]
}

pub fn solve_part2(input: &str) -> Option<usize> {
    const WORD_SEARCH: &str = "MAS";
    let grid = parse_input(input);

    let all_a_iter = grid.iter_with_position()
        .filter(|&(_, chr)| chr == 'A');

    // For all starting positions of the X for XMAS, search for the word in all directions
    let mut num_x_shape_mas_matches = 0;
    for (pos, _x_chr) in all_a_iter {
        let searches = x_shape_word_searches_around_pos(WORD_SEARCH, &pos);
        let num_matches = searches.into_iter()
            .filter(|word_search| is_word_matching(&grid, word_search))
            .count();
        if num_matches == 2 {
            num_x_shape_mas_matches += 1;
        }
    }

    Some(num_x_shape_mas_matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_simple_parsing() {
        let parsed = parse_input(EXAMPLE_INPUT);
        dbg!(&parsed);
        assert_eq!(parsed.data, vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ]);
    }

    #[test]
    fn test_grid_get_in_bound() {
        let grid = Grid::new(vec![
            // x: 0    1    2       / y:
            vec!['a', 'b', 'c'], // line 0
            vec!['d', 'e', 'f'], // line 1
        ]);
        assert_eq!(grid.get_at(&Pos { x: 0, y: 0 }), Some('a'));
        assert_eq!(grid.get_at(&Pos { x: 1, y: 1 }), Some('e'));
    }

    #[test]
    fn test_grid_get_out_of_bound() {
        let grid = Grid::new(vec![
            // x: 0    1    2       / y:
            vec!['a', 'b', 'c'], // line 0
            vec!['d', 'e', 'f'], // line 1
        ]);
        assert_eq!(grid.get_at(&Pos { x: 0, y: -1 }), None);
        assert_eq!(grid.get_at(&Pos { x: -1, y: 0 }), None);
        assert_eq!(grid.get_at(&Pos { x: 0, y: 3 }), None);
        assert_eq!(grid.get_at(&Pos { x: 2, y: 3 }), None);
    }

    #[test]
    fn test_part1_first_2_lines() {
        //  x=.. 0123456789
        // y=0 | MMMSXXMASM
        // y=1 | MSAMXMSMSA
        let result = solve_part1("\
MMMSXXMASM
MSAMXMSMSA
");
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE_INPUT);
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE_INPUT);
        assert_eq!(result, Some(9));
    }
}
