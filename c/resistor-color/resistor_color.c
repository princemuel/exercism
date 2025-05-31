#include "resistor_color.h"
#include <stddef.h>

static const resistor_band_t color_bands[] = {COLORS};

uint8_t color_code(const resistor_band_t color)
{
    return (uint8_t)color;
}

const resistor_band_t *colors(void)
{
    return color_bands;
}
