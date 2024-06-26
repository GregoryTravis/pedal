#include <math.h>

#include "constants.h"

class Playhead {
  public:

  Playhead() : time_in_samples(0) {}

  int get_time_in_samples() {
    return time_in_samples;
  }

  double time_in_seconds() {
    // TODO: un-hardcode sampling rate
    return ((double) time_in_samples) / ((double) SAMPLE_RATE);
  }

  void increment_samples(int delta_samples) {
    time_in_samples += delta_samples;
  }

  void inc() {
    increment_samples(1);
  }

  private:

  int time_in_samples;
};
