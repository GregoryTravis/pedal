#include "hw.h"
#include "arm_math.h"

using namespace std;

#define ARM_SIZE 2048

static bool verbose = false;

static bool arm_initted = false;

extern "C" void do_arm_fft() {
  static float32_t in[ARM_SIZE];
  static float32_t in_copy[ARM_SIZE];
  static float32_t fftBuffer[ARM_SIZE];
  static float32_t out[ARM_SIZE];

  if (verbose) { hw.PrintLine("arm_fft %d\n", ARM_SIZE); }

  static arm_rfft_fast_instance_f32 fftInstance;

  if (verbose) { hw.PrintLine("instance %p\n", &fftInstance); }

  if (!arm_initted) {
    arm_status status = arm_rfft_fast_init_f32(&fftInstance, ARM_SIZE);
    arm_initted = true;
    if (verbose) { hw.PrintLine("status %d\n", status); }
  }

  for (size_t i = 0; i < ARM_SIZE; ++i) {
    in[i] = ((float32_t)i) / ((float32_t)ARM_SIZE);
    in_copy[i] = in[i];
  }

  if (verbose) {
    hw.PrintLine("AAA before");
    for (size_t i = 0; i < ARM_SIZE; ++i) {
      hw.PrintLine("%d %f %f %f\n", i, in[i], fftBuffer[i], out[i]);
    }
  }

  arm_rfft_fast_f32(&fftInstance, in, fftBuffer, 0);

  if (verbose) {
    hw.PrintLine("AAA after fft");
    for (size_t i = 0; i < ARM_SIZE; ++i) {
      hw.PrintLine("%d %f %f %f\n", i, in[i], fftBuffer[i], out[i]);
    }
  }

  arm_rfft_fast_f32(&fftInstance, fftBuffer, out, 1);

  if (verbose) {
    hw.PrintLine("AAA after ifft");
    for (size_t i = 0; i < ARM_SIZE; ++i) {
      hw.PrintLine("%d %f %f %f\n", i, in[i], fftBuffer[i], out[i]);
    }
  }

  // Avergate difference
  float total_diff = 0;
  for (size_t i = 0; i < ARM_SIZE; ++i) {
    total_diff += (out[i] - in_copy[i]);
  }
  float avg_diff = total_diff / ARM_SIZE;
  if (verbose) { hw.PrintLine("AAA diff tot %f avg %f\n", total_diff, avg_diff); }

  float total_fft = 0;
  for (size_t i = 0; i < ARM_SIZE; ++i) {
    total_fft += fftBuffer[i];
  }
  if (verbose) { hw.PrintLine("AAA fft tot %f\n", total_fft); }
}
