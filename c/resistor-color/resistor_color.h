#ifndef RESISTOR_COLOR_H
#define RESISTOR_COLOR_H

#include <stdint.h>
#include <stddef.h>

#define COLORS                         \
    BLACK, BROWN, RED, ORANGE, YELLOW, \
        GREEN, BLUE, VIOLET, GREY, WHITE

typedef enum resistor_band
{
    COLORS
} resistor_band_t;

uint8_t color_code(resistor_band_t color);
const resistor_band_t *colors(void);

#endif
