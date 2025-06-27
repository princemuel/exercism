#ifndef ALLERGIES_H
#define ALLERGIES_H

#include <stdbool.h>
#include <stdint.h>

typedef enum {
  ALLERGEN_EGGS = 0,
  ALLERGEN_PEANUTS,
  ALLERGEN_SHELLFISH,
  ALLERGEN_STRAWBERRIES,
  ALLERGEN_TOMATOES,
  ALLERGEN_CHOCOLATE,
  ALLERGEN_POLLEN,
  ALLERGEN_CATS,
  ALLERGEN_COUNT,
} allergen_t;

typedef struct allergen_list {
  int count;
  bool allergens[ALLERGEN_COUNT];
} allergen_list_t;

bool is_allergic_to(const allergen_t allergen, const uint16_t score);

allergen_list_t get_allergens(int score);

#endif
