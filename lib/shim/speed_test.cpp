#include "speed_test.h"
#include "spew.h"

#define DOT_SIZE 10

static float a[DOT_SIZE];
static float b[DOT_SIZE];
static float accum;

void cpp_speed_test_init() {
  for (int i = 0; i < DOT_SIZE;++i) {
    a[i] = (float) i;
    b[i] = (float) i;
  }
  accum = 0;
}

__attribute__((noinline)) float cpp_f32_dot() {
  float totes = 0.0;

  for (int i = 0; i < DOT_SIZE; ++i) {
    totes += (a[i] * b[i]);
  }

  accum += totes;
  return accum;
}
