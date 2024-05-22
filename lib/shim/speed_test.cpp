#include "speed_test.h"

#define DOT_SIZE 10

void cpp_f32_dot() {
  float a[DOT_SIZE];
  float b[DOT_SIZE];
  float totes = 0.0;

  for (int i = 0; i < DOT_SIZE; ++i) {
    totes += (a[i] * b[i]);
  }
}
