use queen_attack::*;

#[test]
fn queen_with_a_valid_position() {
    let chess_position = ChessPosition::new(2, 2);
    assert!(chess_position.is_some());
}

#[test]
#[ignore]
fn queen_must_have_positive_row() {
    let chess_position = ChessPosition::new(-2, 2);
    assert!(chess_position.is_none());
}

#[test]
#[ignore]
fn queen_must_have_row_on_board() {
    let chess_position = ChessPosition::new(8, 4);
    assert!(chess_position.is_none());
}

#[test]
#[ignore]
fn queen_must_have_positive_column() {
    let chess_position = ChessPosition::new(2, -2);
    assert!(chess_position.is_none());
}

#[test]
#[ignore]
fn queen_must_have_column_on_board() {
    let chess_position = ChessPosition::new(4, 8);
    assert!(chess_position.is_none());
}

#[test]
#[ignore]
fn cannot_attack() {
    let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
    let black_queen = Queen::new(ChessPosition::new(6, 6).unwrap());
    assert!(!white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn can_attack_on_same_row() {
    let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
    let black_queen = Queen::new(ChessPosition::new(2, 6).unwrap());
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn can_attack_on_same_column() {
    let white_queen = Queen::new(ChessPosition::new(4, 5).unwrap());
    let black_queen = Queen::new(ChessPosition::new(2, 5).unwrap());
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn can_attack_on_first_diagonal() {
    let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn can_attack_on_second_diagonal() {
    let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(3, 1).unwrap());
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn can_attack_on_third_diagonal() {
    let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(1, 1).unwrap());
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn can_attack_on_fourth_diagonal() {
    let white_queen = Queen::new(ChessPosition::new(1, 7).unwrap());
    let black_queen = Queen::new(ChessPosition::new(0, 6).unwrap());
    assert!(white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn cannot_attack_if_falling_diagonals_are_only_the_same_when_reflected_across_the_longest_falling_diagonal()
 {
    let white_queen = Queen::new(ChessPosition::new(4, 1).unwrap());
    let black_queen = Queen::new(ChessPosition::new(2, 5).unwrap());
    assert!(!white_queen.can_attack(&black_queen));
}

#[test]
#[ignore]
fn queen_cannot_attack_itself() {
    let queen = Queen::new(ChessPosition::new(4, 4).unwrap());
    assert!(!queen.can_attack(&queen));
}

#[test]
#[ignore]
fn queen_cannot_attack_same_position_different_instances() {
    let white_queen = Queen::new(ChessPosition::new(3, 5).unwrap());
    let black_queen = Queen::new(ChessPosition::new(3, 5).unwrap());
    assert!(!white_queen.can_attack(&black_queen));
}

// Boundary position tests
#[test]
#[ignore]
fn can_attack_from_corner_positions() {
    // Top-left corner to bottom-right corner (main diagonal)
    let corner_queen = Queen::new(ChessPosition::new(0, 0).unwrap());
    let opposite_corner = Queen::new(ChessPosition::new(7, 7).unwrap());
    assert!(corner_queen.can_attack(&opposite_corner));
}

#[test]
#[ignore]
fn can_attack_from_other_corner_diagonal() {
    // Top-right corner to bottom-left corner (anti-diagonal)
    let corner_queen = Queen::new(ChessPosition::new(0, 7).unwrap());
    let opposite_corner = Queen::new(ChessPosition::new(7, 0).unwrap());
    assert!(corner_queen.can_attack(&opposite_corner));
}

#[test]
#[ignore]
fn can_attack_across_full_board_rank() {
    // Leftmost to rightmost on same rank
    let left_queen = Queen::new(ChessPosition::new(4, 0).unwrap());
    let right_queen = Queen::new(ChessPosition::new(4, 7).unwrap());
    assert!(left_queen.can_attack(&right_queen));
}

#[test]
#[ignore]
fn can_attack_across_full_board_file() {
    // Top to bottom on same file
    let top_queen = Queen::new(ChessPosition::new(0, 3).unwrap());
    let bottom_queen = Queen::new(ChessPosition::new(7, 3).unwrap());
    assert!(top_queen.can_attack(&bottom_queen));
}

#[test]
#[ignore]
fn can_attack_adjacent_rank() {
    let queen1 = Queen::new(ChessPosition::new(3, 3).unwrap());
    let queen2 = Queen::new(ChessPosition::new(3, 4).unwrap());
    assert!(queen1.can_attack(&queen2));
}

#[test]
#[ignore]
fn can_attack_adjacent_file() {
    let queen1 = Queen::new(ChessPosition::new(3, 3).unwrap());
    let queen2 = Queen::new(ChessPosition::new(4, 3).unwrap());
    assert!(queen1.can_attack(&queen2));
}

#[test]
#[ignore]
fn can_attack_adjacent_diagonal() {
    let queen1 = Queen::new(ChessPosition::new(3, 3).unwrap());
    let queen2 = Queen::new(ChessPosition::new(4, 4).unwrap());
    assert!(queen1.can_attack(&queen2));
}

// Position validation boundary tests
#[test]
#[ignore]
fn valid_position_at_zero_zero() {
    let chess_position = ChessPosition::new(0, 0);
    assert!(chess_position.is_some());
}

#[test]
#[ignore]
fn valid_position_at_seven_seven() {
    let chess_position = ChessPosition::new(7, 7);
    assert!(chess_position.is_some());
}

#[test]
#[ignore]
fn invalid_position_just_below_zero_rank() {
    let chess_position = ChessPosition::new(-1, 0);
    assert!(chess_position.is_none());
}

#[test]
#[ignore]
fn invalid_position_just_below_zero_file() {
    let chess_position = ChessPosition::new(0, -1);
    assert!(chess_position.is_none());
}

#[test]
#[ignore]
fn invalid_position_just_above_seven_rank() {
    let chess_position = ChessPosition::new(8, 0);
    assert!(chess_position.is_none());
}

#[test]
#[ignore]
fn invalid_position_just_above_seven_file() {
    let chess_position = ChessPosition::new(0, 8);
    assert!(chess_position.is_none());
}

// Test with extreme values to ensure no overflow
#[test]
#[ignore]
fn invalid_position_with_large_negative_values() {
    let chess_position = ChessPosition::new(i32::MIN, i32::MIN);
    assert!(chess_position.is_none());
}

#[test]
#[ignore]
fn invalid_position_with_large_positive_values() {
    let chess_position = ChessPosition::new(i32::MAX, i32::MAX);
    assert!(chess_position.is_none());
}
