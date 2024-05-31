#include "speed_test.h"
#include "circbuf.h"
#include "spew.h"

#define DOT_SIZE 10
#define CIRC_NEW_ADD 3
#define CIRC_SUM_SIZE 7

static float a[DOT_SIZE];
static float b[DOT_SIZE];
static float accum;

static CircBuf cbuf(DOT_SIZE);

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

  accum = totes;
  return accum;
}

__attribute__((noinline)) float cpp_f32_circsum() {
  for (int i = 0; i < CIRC_NEW_ADD; ++i) {
    cbuf.push(a[i]);
  }
  int sum_offset = (DOT_SIZE - CIRC_SUM_SIZE) / 2;
  int sum_end = sum_offset + CIRC_SUM_SIZE;
  //assert(sum_offset >= 0 && sum_offset <= DOT_SIZE);
  //assert(sum_end >= 0 && sum_end <= DOT_SIZE);
  float totes = 0.0;
  for (int i = sum_offset; i < sum_end; ++i) {
    totes += cbuf.get(i);
  }
  accum = totes;
  return accum;
}
