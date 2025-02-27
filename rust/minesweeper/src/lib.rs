pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();

    if height == 0 {
        return vec![];
    }

    (0..height)
        .map(|row| {
            let width = minefield[row].len();

            (0..width)
                .map(|col| {
                    if is_mine(minefield, row, col) {
                        '*'
                    } else {
                        let count = count_adjacent_mines(minefield, row, col);

                        if count > 0 {
                            char::from_digit(count, 10).unwrap()
                        } else {
                            ' '
                        }
                    }
                })
                .collect()
        })
        .collect()
}

const NEIGHBOR_DELTAS: [(isize, isize); 8] =
    [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn count_adjacent_mines(minefield: &[&str], row: usize, col: usize) -> u32 {
    NEIGHBOR_DELTAS
        .iter()
        .filter_map(|&(dr, dc)| {
            let (new_row, new_col) = (row as isize + dr, col as isize + dc);

            (new_row >= 0 && new_col >= 0)
                .then_some((new_row as usize, new_col as usize))
        })
        .filter(|&(r, c)| is_mine(minefield, r, c))
        .count() as u32
}

fn is_mine(minefield: &[&str], row: usize, col: usize) -> bool {
    minefield
        .get(row)
        .and_then(|r| r.as_bytes().get(col))
        .is_some_and(|&c| c == b'*')
}
