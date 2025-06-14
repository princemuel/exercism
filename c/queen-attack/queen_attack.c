#include "queen_attack.h"

#include <stdbool.h>
#include <stdlib.h>

static bool is_valid_position(position_t pos) {
  return pos.row < 8 && pos.column < 8;
}

static bool has_valid_positions(position_t pos_1, position_t pos_2) {
  if (!is_valid_position(pos_1) || !is_valid_position(pos_2)) return false;
  return !(pos_1.row == pos_2.row && pos_1.column == pos_2.column);
}

static bool is_on_straight_or_diagonal(position_t pos_1, position_t pos_2) {
  // Direction Vector
  int dr = pos_2.row - pos_1.row;
  int dc = pos_2.column - pos_1.column;
  return (dr == 0 || dc == 0 || abs(dr) == abs(dc));
}

attack_status_t can_attack(position_t queen_1, position_t queen_2) {
  return (!has_valid_positions(queen_1, queen_2))       ? INVALID_POSITION
         : is_on_straight_or_diagonal(queen_1, queen_2) ? CAN_ATTACK
                                                        : CAN_NOT_ATTACK;
}
