#include "daisy_seed.h"
#include "daisysp.h"

using namespace daisy;
using namespace daisysp;

DaisySeed hw;

extern "C" {
  typedef void *PatchPtr;
  void rust_process_audio(PatchPtr patch, const float* const* in_ptr, float **out_ptr, size_t len);
  PatchPtr get_patch();
  float use_patch(PatchPtr);
  size_t get_size();
  void rust_setup();
}

static PatchPtr thePatchPtr;

float inl, inr, outl, outr;
int frames=0;

void copyInToOut(AudioHandle::InputBuffer in, AudioHandle::OutputBuffer out, size_t size) {
  for (size_t i = 0; i < size; i++) {
          out[0][i] = in[1][i];
          out[1][i] = in[0][i];
  }
}

void AudioCallback(AudioHandle::InputBuffer in, AudioHandle::OutputBuffer out, size_t size)
{
  rust_process_audio(thePatchPtr, in, out, size);
  //copyInToOut(in, out, size);

  inl = in[0][0];
  inr = in[1][0];
  outl = out[0][0];
  outr = out[1][0];
  frames++;
}

void initLogging() {
  hw.StartLog(true);
  hw.PrintLine("Pedal!");
}

extern "C" int cpp_main(void)
{
	hw.Init();
        initLogging();
	hw.SetAudioBlockSize(4); // number of samples handled per callback
	hw.SetAudioSampleRate(SaiHandle::Config::SampleRate::SAI_48KHZ);
	hw.StartAudio(AudioCallback);

        // TODO move this earlier
        rust_setup();

        hw.PrintLine("PatchPtr size %d", get_size());
        thePatchPtr = get_patch();
        hw.PrintLine("PatchPtr %p", thePatchPtr);
        float pf = use_patch(thePatchPtr);
        hw.PrintLine("PatchPtr foo %f", pf);

	while(1) {
          System::Delay(500);
          hw.PrintLine("dl %f %f %f %f %d", inl, inr, outl, outr, frames);
        }
	//while(1) {}
}
