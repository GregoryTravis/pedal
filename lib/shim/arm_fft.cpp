#include "arm_math.h"

#include "constants.h"

using namespace std;

static bool arm_initted = false;
static arm_rfft_fast_instance_f32 fftInstance;

inline void arm_init_if_not() {
  if (!arm_initted) {
    arm_status status = arm_rfft_fast_init_f32(&fftInstance, FFT_SIZE);
    assert(status == 0);
    arm_initted = true;
  }
}

extern "C" void do_arm_fft(float *input, float *output) {
  arm_init_if_not();
  arm_rfft_fast_f32(&fftInstance, input, output, 0);
}

extern "C" void do_arm_ifft(float *input, float *output) {
  arm_init_if_not();
  arm_rfft_fast_f32(&fftInstance, input, output, 1);
}
