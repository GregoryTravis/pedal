#include "patch.h"

#include "load.h"

using namespace daisy;

Vibrato *vibrato = nullptr;
Playhead playhead;

void CppVibratoAudioCallback(AudioHandle::InputBuffer in, AudioHandle::OutputBuffer out, size_t size)
{
  load_before();
  vibrato->cpp_process_audio(in[1], out[1], size, playhead);
  for (size_t i = 0; i < size; ++i) {
    out[0][i] = out[1][i];
  }
  playhead.increment_samples(size);
  load_after();
}

void patch_setup() {
  vibrato = new Vibrato(400, 0.10);
}
