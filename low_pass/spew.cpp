#include "spew.h"

extern "C" void spew_int_c(int x) {
  hw.Print("%d", x);
}

extern "C" void spew_size_t_c(size_t x) {
  hw.Print("%d", x);
}

extern "C" void spew_float_c(float x) {
  hw.Print("%f", x);
}

extern "C" void spew_string_c(char *s) {
  hw.Print("%s", s);
}

extern "C" void spew_space_c() {
  hw.Print(" ");
}

extern "C" void spew_newline_c() {
  hw.PrintLine("");
}
