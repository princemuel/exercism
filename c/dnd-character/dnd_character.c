#include "dnd_character.h"

#include <math.h>
#include <stdbool.h>
#include <stdlib.h>
#include <time.h>

int ability(void) {
  // Seed random number generator once
  static bool seed_initialized = false;
  if (!seed_initialized) {
    srand((unsigned int)time(NULL));
    seed_initialized = true;
  }

  // Roll 4d6, drop lowest - single pass approach
  int sum = 0;
  int min_roll = 7;

  for (int i = 0; i < 4; i++) {
    int roll = (rand() % 6) + 1;
    sum += roll;
    if (roll < min_roll) min_roll = roll;
  }

  return sum - min_roll;
}

int modifier(int score) { return (int)floor((score - 10) / 2.0); }

dnd_character_t make_dnd_character(void) {
  dnd_character_t character = {.strength = ability(),
                               .dexterity = ability(),
                               .constitution = ability(),
                               .intelligence = ability(),
                               .wisdom = ability(),
                               .charisma = ability()};

  character.hitpoints = 10 + modifier(character.constitution);
  return character;
}
