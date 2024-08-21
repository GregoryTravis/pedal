#include "daisy_seed.h"
#include "daisysp.h"
#include "arm_math.h"

#include "hw.h"

// Use the daisy namespace to prevent having to type
// daisy:: before all libdaisy functions
using namespace daisy;
using namespace daisysp;

#define SIZE 2048
#define EXTRA 0

float32_t in[SIZE+EXTRA];
float32_t fftBuffer[SIZE+EXTRA];
float32_t out[SIZE+EXTRA];

extern "C" void do_fft() {
  arm_rfft_fast_instance_f32* fftInstance = new arm_rfft_fast_instance_f32;

  hw.PrintLine("instance %p\n", fftInstance);

  arm_status status = arm_rfft_fast_init_f32(fftInstance, SIZE);

  hw.PrintLine("status %d\n", status);

  for (size_t i = 0; i < SIZE; ++i) {
    in[i] = ((float32_t)i) / ((float32_t)SIZE);
  }

  hw.PrintLine("AAA before");
  for (size_t i = 0; i < SIZE; ++i) {
    hw.PrintLine("%d %f %f %f\n", i, in[i], fftBuffer[i], out[i]);
  }

  arm_rfft_fast_f32(fftInstance, in, fftBuffer, 0);

  hw.PrintLine("AAA after fft");
  for (size_t i = 0; i < SIZE; ++i) {
    hw.PrintLine("%d %f %f %f\n", i, in[i], fftBuffer[i], out[i]);
  }

  arm_rfft_fast_f32(fftInstance, fftBuffer, out, 1);

  hw.PrintLine("AAA after ifft");
  for (size_t i = 0; i < SIZE; ++i) {
    hw.PrintLine("%d %f %f %f\n", i, in[i], fftBuffer[i], out[i]);
  }
}
