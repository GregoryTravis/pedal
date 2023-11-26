#include "daisy_seed.h"

using namespace daisy;

#define CPU_LOAD 1

extern void load_init();
extern void load_before();
extern void load_after();
extern "C" void load_spew();
