#include <assert.h>

#include "daisy_seed.h"

#include "hw.h"
#include "load.h"
#include "spew.h"
#include "vibrato.h"

using namespace daisy;

Vibrato *vibrato = nullptr;
Playhead playhead;

extern "C" {
  void rust_process_audio_stub(const float* const* in_ptr, float **out_ptr, size_t len);
  void patch_main();
  int PEDAL_MAIN();
}

// (libDaisy/src/hid/audio.h)
// Non-Interleaving output buffer
// Arranged by float[chn][sample] 
// Left 0, Right 1
// The mono pedal is right only
// typedef const float* const* InputBuffer;
// typedef float** OutputBuffer;
void AudioCallback(AudioHandle::InputBuffer in, AudioHandle::OutputBuffer out, size_t size)
{
  load_before();
  rust_process_audio_stub(in, out, size);
  load_after();
}

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

#define TEST_CPP 1

void initLogging() {
  hw.StartLog(true);
  hw.PrintLine("Pedal!");
}

extern "C" void UnsafeDelay(uint32_t delay_ms) {
  System::Delay(delay_ms);
}

extern "C" int cpp_main(void)
{
	hw.Init();
  initLogging();
	hw.SetAudioBlockSize(4); // number of samples handled per callback
	hw.SetAudioSampleRate(SaiHandle::Config::SampleRate::SAI_48KHZ);

  assert(hw.audio_handle.GetChannels() == 2);

  load_init();

#if TEST_CPP
	//hw.StartAudio(CppSpeedAudioCallback);
	hw.StartAudio(CppVibratoAudioCallback);
#else
	hw.StartAudio(AudioCallback);
#endif

  patch_main();
  while(1) {} // Just in case we fall through
}

int main() {
  vibrato = new Vibrato(400, 0.10);
  PEDAL_MAIN();
}
