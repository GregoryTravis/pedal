#include <stdio.h>
#include <assert.h>
#include <math.h>

#ifdef STANDALONE
#include <algorithm>
#include "AudioFile.h"
#endif

#include "vibrato.h"

float sinc(float x) {
    // TODO how on earth does this work?
    float small = 0.000000000000000000000000000000000000000000001;
    if (x < small || x > -small) {
        return 1.0;
    } else {
        return sin(x) / x;
    }
}

void Vibrato::cpp_process_audio(
    const float *input_slice,
    float *output_slice,
    size_t size,
    Playhead playhead) {
  for (size_t i=0; i < size; ++i) {
      cbuf.push(input_slice[i]);
      double tis = playhead.time_in_seconds();
      double vibrato_deviation = sin(
          tis * ((double) vibrato_frequency) * 2.0 * ((double) M_PI)) * ((double) max_sample_deviation);
      // Fractional playhead
      float fph = ((float) now_index) + vibrato_deviation;
      float window_low_f = fph - ((float) NUM_SINC_TAPS_ONE_SIDE);
      float window_high_f = fph + ((float) NUM_SINC_TAPS_ONE_SIDE);
      assert(window_low_f > 0.0);
      assert(window_high_f < (float) buffer_length);
      size_t window_low_i = (size_t) ceil(window_low_f);
      size_t window_high_i = (size_t) floor(window_high_f);
      assert(window_low_i < window_high_i);
      float convolution_sum = 0.0;
      for (size_t si = window_low_i; si < window_high_i+1; ++si) {
          float sinc_x = fph - ((float) si);
          float sinc_value = sinc(sinc_x);
          float si_sample = cbuf.get(si);
          convolution_sum += sinc_value * si_sample;
      }
      convolution_sum /= 2.0;
      if (!(convolution_sum <= 1.0 && convolution_sum >= -1.0)) {
          printf("Overflow %f\n", convolution_sum);
      }
      output_slice[i] = convolution_sum;
      playhead.inc();
  }
}

#ifdef STANDALONE
int main(int argc, char *argv[]) {
  printf("float is %lu\n", sizeof(float));
  printf("double is %lu\n", sizeof(double));
  printf("int is %lu\n", sizeof(int));

  char *infile = argv[1];
  char *outfile = argv[2];

  AudioFile<float> audioFile;
  audioFile.load(infile);

  assert(audioFile.isMono());
  assert(audioFile.getBitDepth() == 16);

  int numSamples = audioFile.getNumSamplesPerChannel();
  assert(numSamples > 0);
  int bufSize = 4;

  AudioFile<float> outAudioFile;
  outAudioFile.setAudioBufferSize(1, numSamples);
  outAudioFile.setBitDepth(audioFile.getBitDepth());
  outAudioFile.setSampleRate(audioFile.getSampleRate());
  AudioFile<float>::AudioBuffer buffer;
  buffer.resize(1);
  buffer[0].resize(numSamples);
  outAudioFile.setAudioBuffer(buffer);

  Vibrato vibrato(400, 1.0);
  Playhead playhead;

  int current = 0;
  while (current < numSamples) {
    float *inp = &audioFile.samples[0][current];
    float *outp = &outAudioFile.samples[0][current];
    size_t remaining = std::min(numSamples - current, bufSize);
    vibrato.cpp_process_audio(inp, outp, remaining, playhead);
    playhead.increment_samples(remaining);
    current += remaining;
  }

  outAudioFile.save(outfile);
}
#endif

