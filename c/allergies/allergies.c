#include "allergies.h"

#include <stdint.h>

bool is_allergic_to(const allergen_t allergen, const uint16_t score) {
  const uint8_t mask = score & 0xFF;
  return (mask & (1 << allergen)) != 0;
}

allergen_list_t get_allergens(int score) {
  allergen_list_t result = {0};
  score = score & 0xFF;

  for (int i = 0; i < ALLERGEN_COUNT; i++) {
    if (score & (1 << i)) {
      result.allergens[i] = true;
      result.count++;
    }
  }

  return result;
}
