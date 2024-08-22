//#include <chrono>

#include "daisy_seed.h"
#include "daisysp.h"

#include "arm_math.h"
#include "shy_fft.h"
#include "stmlib/stmlib.h"

#include "hw.h"

// Use the daisy namespace to prevent having to type
// daisy:: before all libdaisy functions
using namespace std;
using namespace daisy;
using namespace daisysp;

#define ARM_SIZE 2048
#define EXTRA 0

extern "C" void do_arm_fft() {
  /*
  auto now = chrono::system_clock::now();
  auto duration = now.time_since_epoch();

  // Convert duration to milliseconds
  auto milliseconds
      = chrono::duration_cast<chrono::milliseconds>(
            duration)
            .count();
  long ms = milliseconds;
  hw.PrintLine("time %d\n", ms);
  return;
  */

  static float32_t in[ARM_SIZE+EXTRA];
  static float32_t in_copy[ARM_SIZE+EXTRA];
  static float32_t fftBuffer[ARM_SIZE+EXTRA];
  static float32_t out[ARM_SIZE+EXTRA];

  hw.PrintLine("arm_fft %d\n", ARM_SIZE);

  arm_rfft_fast_instance_f32* fftInstance = new arm_rfft_fast_instance_f32;

  hw.PrintLine("instance %p\n", fftInstance);

  arm_status status = arm_rfft_fast_init_f32(fftInstance, ARM_SIZE);

  hw.PrintLine("status %d\n", status);

  for (size_t i = 0; i < ARM_SIZE; ++i) {
    in[i] = ((float32_t)i) / ((float32_t)ARM_SIZE);
    in_copy[i] = in[i];
  }

  hw.PrintLine("AAA before");
  for (size_t i = 0; i < ARM_SIZE; ++i) {
    hw.PrintLine("%d %f %f %f\n", i, in[i], fftBuffer[i], out[i]);
  }

  arm_rfft_fast_f32(fftInstance, in, fftBuffer, 0);

  hw.PrintLine("AAA after fft");
  for (size_t i = 0; i < ARM_SIZE; ++i) {
    hw.PrintLine("%d %f %f %f\n", i, in[i], fftBuffer[i], out[i]);
  }

  arm_rfft_fast_f32(fftInstance, fftBuffer, out, 1);

  hw.PrintLine("AAA after ifft");
  for (size_t i = 0; i < ARM_SIZE; ++i) {
    hw.PrintLine("%d %f %f %f\n", i, in[i], fftBuffer[i], out[i]);
  }

  // Avergate difference
  float total_diff = 0;
  for (size_t i = 0; i < ARM_SIZE; ++i) {
    total_diff += (out[i] - in_copy[i]);
  }
  float avg_diff = total_diff / ARM_SIZE;
  hw.PrintLine("AAA tot %f avg %f\n", total_diff, avg_diff);
}

#define SHY_SIZE 16

typedef ShyFFT<float, SHY_SIZE, RotationPhasor> FFT;

extern "C" void do_shy_fft() {
  static float32_t in[SHY_SIZE+EXTRA];
  static float32_t in_copy[ARM_SIZE+EXTRA];
  static float32_t fftBuffer[SHY_SIZE+EXTRA];
  static float32_t out[SHY_SIZE+EXTRA];

  hw.PrintLine("shy_fft %d\n", SHY_SIZE);

  FFT fft;
  fft.Init();

  for (size_t i = 0; i < SHY_SIZE; ++i) {
    in[i] = ((float32_t)i) / ((float32_t)SHY_SIZE);
    in_copy[i] = in[i];
  }

  hw.PrintLine("AAA before");
  for (size_t i = 0; i < SHY_SIZE; ++i) {
    hw.PrintLine("%d %f %f %f\n", i, in[i], fftBuffer[i], out[i]);
  }

  fft.Direct(in, fftBuffer);

  hw.PrintLine("AAA after fft");
  for (size_t i = 0; i < SHY_SIZE; ++i) {
    hw.PrintLine("%d %f %f %f\n", i, in[i], fftBuffer[i], out[i]);
  }

  fft.Inverse(fftBuffer, out);

  hw.PrintLine("AAA after ifft");
  for (size_t i = 0; i < SHY_SIZE; ++i) {
    hw.PrintLine("%d %f %f %f\n", i, in[i], fftBuffer[i], out[i]);
  }

  // Avergate difference
  float total_diff = 0;
  for (size_t i = 0; i < SHY_SIZE; ++i) {
    // Shy does not normalize
    total_diff += ((out[i] / SHY_SIZE) - in_copy[i]);
  }
  float avg_diff = total_diff / ARM_SIZE;
  hw.PrintLine("AAA tot %f avg %f\n", total_diff, avg_diff);
}
