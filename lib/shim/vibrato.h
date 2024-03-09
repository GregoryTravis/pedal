#include "circbuf.h"
#include "playhead.h"

// The sinc convolution window is twice this.
#define NUM_SINC_TAPS_ONE_SIDE 3

// Add this many samples on either side to prevent under/overruns in production. Should
// pass rigorous testing with this set to 0, though.
#define GUARD_SAMPLES 1

class Vibrato {
  public:

  Vibrato(size_t max_sample_deviation, float vibrato_frequency)
    : max_sample_deviation(max_sample_deviation),
      vibrato_frequency(vibrato_frequency),
      buffer_length(2 * (max_sample_deviation + NUM_SINC_TAPS_ONE_SIDE + GUARD_SAMPLES) + 1),
      now_index(max_sample_deviation + NUM_SINC_TAPS_ONE_SIDE + GUARD_SAMPLES),
      cbuf(buffer_length) {}

  void cpp_process_audio(
    const float *input_slice,
    float *output_slice,
    size_t size,
    Playhead playhead);

  private:
  // The fractional playhead can only deviate from the regular one by this much on either side.
  // Range is *exclusive*. -- TODO ??
  size_t max_sample_deviation;
  // Hz
  float vibrato_frequency;

  size_t buffer_length;
  size_t now_index;
  CircBuf cbuf;
};

