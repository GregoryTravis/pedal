#include "daisy_seed.h"

using namespace daisy;

DaisySeed hw;

extern "C" {
  typedef void *PatchPtr;
  void rust_process_audio(PatchPtr patch, const float* const* in_ptr, float **out_ptr, size_t len);
  void rust_patch_main(PatchPtr patch);
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

template <typename... VA>
static void PrintLine(const char* format, VA... va)
{
    hw.PrintLine(format, va...);
}

void Delay(uint32_t delay_ms) {
  System::Delay(500);
}

extern "C" int cpp_main(void)
{
        // TODO move this earlier
        rust_setup();

	hw.Init();
        initLogging();
	hw.SetAudioBlockSize(4); // number of samples handled per callback
	hw.SetAudioSampleRate(SaiHandle::Config::SampleRate::SAI_48KHZ);
	hw.StartAudio(AudioCallback);

        hw.PrintLine("PatchPtr size %d", get_size());
        thePatchPtr = get_patch();
        hw.PrintLine("PatchPtr %p", thePatchPtr);
        float pf = use_patch(thePatchPtr);
        hw.PrintLine("PatchPtr foo %f", pf);

       while(1) {
          hw.PrintLine("dl %f %f %f %f %d", inl, inr, outl, outr, frames);
          System::Delay(500);
        }
       //while(1) {}
        //rust_patch_main();
}
