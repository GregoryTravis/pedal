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
// typedef const float* const* InputBuffer;
// typedef float** OutputBuffer;
void AudioCallback(AudioHandle::InputBuffer in, AudioHandle::OutputBuffer out, size_t size)
{
  load_before();
  //rust_process_audio_stub(in, out, size);
  vibrato->cpp_process_audio(in[1], out[1], size, playhead);
  for (size_t i = 0; i < size; ++i) {
    out[0][i] = out[1][i];
  }
  playhead.increment_samples(size);
  load_after();
}

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

  load_init();

	hw.StartAudio(AudioCallback);

  patch_main();
  while(1) {} // Just in case we fall through
}

int main() {
  spew_string_c("float");
  spew_int_c(sizeof(float));
  spew_string_c("double");
  spew_int_c(sizeof(double));
  spew_string_c("int");
  spew_int_c(sizeof(int));
  vibrato = new Vibrato(400, 0.10);
  PEDAL_MAIN();
}
