#include "shy_fft.h"

#include "constants.h"
#include "hw.h"

using namespace std;

typedef ShyFFT<float, FFT_SIZE, RotationPhasor> FFT;

static bool shy_initted = false;
static FFT fft;

inline void shy_init_if_not() {
  if (!shy_initted) {
    fft.Init();
    shy_initted = true;
  }
}

extern "C" void do_shy_fft(float *input, float *output) {
  shy_init_if_not();
  fft.Direct(input, output);
}

extern "C" void do_shy_ifft(float *input, float *output) {
  shy_init_if_not();
  fft.Inverse(input, output);
}
