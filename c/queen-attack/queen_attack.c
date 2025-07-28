#include "queen_attack.h"
#include <stdbool.h>
#include <stdlib.h>

static bool is_valid_position(position_t pos) {
  return pos.row < BOARD_SIZE && pos.column < BOARD_SIZE;
}

static bool are_same_position(position_t pos_1, position_t pos_2) {
  return pos_1.row == pos_2.row && pos_1.column == pos_2.column;
}

static bool is_on_attack_line(position_t pos_1, position_t pos_2) {
  // Direction Vector
  int dr = abs((int)pos_2.row - (int)pos_1.row);
  int dc = abs((int)pos_2.column - (int)pos_1.column);
  return (dr == 0 || dc == 0 || (dr == dc));
}

attack_status_t can_attack(position_t queen_1, position_t queen_2) {
  if (!is_valid_position(queen_1) || !is_valid_position(queen_2)) {
    return INVALID_POSITION;
  }

  if (are_same_position(queen_1, queen_2)) {
    return INVALID_POSITION;
  }

  return is_on_attack_line(queen_1, queen_2) ? CAN_ATTACK : CAN_NOT_ATTACK;
}
