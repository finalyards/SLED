#pragma once

// wrap.h

#include "led_strip.h"
#include "led_strip_rmt.h"

// A wrap around a macro that would otherwise not make it through.
static inline led_color_component_format_t wrap_LED_STRIP_COLOR_COMPONENT_FMT_GRB() {
    return LED_STRIP_COLOR_COMPONENT_FMT_GRB;
}
