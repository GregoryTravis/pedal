#include "daisy_seed.h"

#include "hw.h"
#include "load.h"
#include "spew.h"

using namespace daisy;

extern "C" {
  void rust_process_audio_stub(const float* const* in_ptr, float **out_ptr, size_t len);
  void patch_main();
  int PEDAL_MAIN();
}

void AudioCallback(AudioHandle::InputBuffer in, AudioHandle::OutputBuffer out, size_t size)
{
  load_before();
  rust_process_audio_stub(in, out, size);
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

	//hw.StartAudio(AudioCallback);

  patch_main();
  while(1) {} // Just in case we fall through
}

int main() {
  PEDAL_MAIN();
}
