#include "daisy_seed.h"

using namespace daisy;

DaisySeed hw;

extern "C" {
  typedef void *PatchPtr;
  void rust_process_audio_stub(PatchPtr patch, const float* const* in_ptr, float **out_ptr, size_t len);
  void rust_process_audio(PatchPtr patch, const float* const* in_ptr, float **out_ptr, size_t len);
  void patch_main(PatchPtr patch);
  //void rust_patch_main(PatchPtr patch);
  PatchPtr get_patch();
  float use_patch(PatchPtr);
  size_t get_size();
  void rust_setup();
}

extern "C" void spew_int_c(int x) {
  hw.PrintLine("%d", x);
}

extern "C" void spew_float_c(float x) {
  hw.PrintLine("%f", x);
}

extern "C" void spew_string_c(char *s) {
  //hw.PrintLine("%s", s);
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
  rust_process_audio_stub(thePatchPtr, in, out, size);
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

extern "C" void PrintLine(const char* format)
{
    hw.PrintLine(format);
}

extern "C" void ping() {
  hw.PrintLine("ping");
}

extern "C" void UnsafeDelay(uint32_t delay_ms) {
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

        hw.PrintLine("PatchPtr size %d", get_size());
        thePatchPtr = get_patch();
        hw.PrintLine("PatchPtr %p", thePatchPtr);
        float pf = use_patch(thePatchPtr);
        hw.PrintLine("PatchPtr foo %f", pf);

	hw.StartAudio(AudioCallback);

        patch_main(thePatchPtr);

        while(1) {
          hw.PrintLine("dl %f %f %f %f %d", inl, inr, outl, outr, frames);
          System::Delay(500);
        }
       //while(1) {}
}
