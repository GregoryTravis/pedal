#include "hw.h"
#include "shy_fft.h"

// NOTE: this is not part of shy_fft.h; it merely uses it.

using namespace std;

#define SHY_SIZE 2048

static bool verbose = false;

typedef ShyFFT<float, SHY_SIZE, RotationPhasor> FFT;

static bool shy_initted = false;

extern "C" void do_shy_fft() {
  static float in[SHY_SIZE];
  static float in_copy[SHY_SIZE];
  static float fftBuffer[SHY_SIZE];
  static float out[SHY_SIZE];

  if (verbose) { hw.PrintLine("shy_fft %d\n", SHY_SIZE); }

  static FFT fft;

  if (!shy_initted) {
    fft.Init();
    shy_initted = true;
  }

  for (size_t i = 0; i < SHY_SIZE; ++i) {
    in[i] = ((float)i) / ((float)SHY_SIZE);
    in_copy[i] = in[i];
  }

  if (verbose) {
    hw.PrintLine("AAA before");
    for (size_t i = 0; i < SHY_SIZE; ++i) {
      hw.PrintLine("%d %f %f %f\n", i, in[i], fftBuffer[i], out[i]);
    }
  }

  fft.Direct(in, fftBuffer);

  if (verbose) {
    hw.PrintLine("AAA after fft");
    for (size_t i = 0; i < SHY_SIZE; ++i) {
      hw.PrintLine("%d %f %f %f\n", i, in[i], fftBuffer[i], out[i]);
    }
  }

  fft.Inverse(fftBuffer, out);

  if (verbose) {
    hw.PrintLine("AAA after ifft");
    for (size_t i = 0; i < SHY_SIZE; ++i) {
      hw.PrintLine("%d %f %f %f\n", i, in[i], fftBuffer[i], out[i]);
    }
  }

  // Avergate difference
  float total_diff = 0;
  for (size_t i = 0; i < SHY_SIZE; ++i) {
    // Shy does not normalize
    total_diff += ((out[i] / SHY_SIZE) - in_copy[i]);
  }
  float avg_diff = total_diff / SHY_SIZE;
  if (verbose) { hw.PrintLine("AAA tot %f avg %f\n", total_diff, avg_diff); }

  float total_fft = 0;
  for (size_t i = 0; i < SHY_SIZE; ++i) {
    total_fft += fftBuffer[i];
  }
  if (verbose) { hw.PrintLine("AAA fft tot %f\n", total_fft); }
}
