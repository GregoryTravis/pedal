#include "daisy_seed.h"

#include "hw.h"
#include "load.h"
#include "spew.h"

using namespace daisy;

extern "C" {
  typedef void *PatchPtr;
  void rust_process_audio_stub(PatchPtr patch, const float* const* in_ptr, float **out_ptr, size_t len);
  void patch_main(PatchPtr patch);
  //void rust_patch_main(PatchPtr patch);
  PatchPtr get_patch();
  float use_patch(PatchPtr);
  size_t get_size();
}

static PatchPtr thePatchPtr;

void AudioCallback(AudioHandle::InputBuffer in, AudioHandle::OutputBuffer out, size_t size)
{
  load_before();
  rust_process_audio_stub(thePatchPtr, in, out, size);
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

  hw.PrintLine("PatchPtr size %d", get_size());
  thePatchPtr = get_patch();
  hw.PrintLine("PatchPtr %p", thePatchPtr);
  float pf = use_patch(thePatchPtr);
  hw.PrintLine("PatchPtr foo %f", pf);

  load_init();

	hw.StartAudio(AudioCallback);

  patch_main(thePatchPtr);
  while(1) {} // Just in case we fall through
}
