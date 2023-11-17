#include "daisy_seed.h"

using namespace daisy;

DaisySeed hw;

extern "C" {
  typedef void *PatchPtr;
  void rust_process_audio_stub(PatchPtr patch, const float* const* in_ptr, float **out_ptr, size_t len);
  void rust_process_audio(PatchPtr patch, const float* const* in_ptr, float **out_ptr, size_t len);
  void patch_main(PatchPtr patch);
  //void rust_patch_main(PatchPtr patch);
  PatchPtr get_patch(size_t g);
  float use_patch(PatchPtr);
  size_t get_size();
  void rust_setup();
}

extern "C" void spew_ptr_c(void *p) {
  printf("c++ size %d\n", sizeof(p));
  printf("c++ size %d\n", sizeof(void *));
  hw.PrintLine("%p", p);
}

extern "C" void spew_int_c(int x) {
  hw.PrintLine("%d", x);
}

extern "C" void spew_size_t_c(size_t x) {
  hw.PrintLine("%d", x);
}

extern "C" void spew_float_c(float x) {
  hw.PrintLine("%f", x);
}

extern "C" void spew_string_c(char *s) {
  hw.PrintLine("spew_string_c %p %s", s, s);
}

extern "C" void spew_space_c() {
  hw.PrintLine(" ");
}

extern "C" void spew_newline_c() {
  hw.PrintLine("");
}

static PatchPtr thePatchPtr;

float inl, inr, outl, outr;
int frames=0;

const float *pinl = 0;
const float *pinr = 0;
const float *poutl = 0;
const float *poutr = 0;

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

  pinl = &in[0][0];
  pinr = &in[1][0];
  poutl = &out[0][0];
  poutr = &out[1][0];
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

void dump_allocator(const char *s) {
  //return;
  uint32_t *emb = (uint32_t*) 0x20000004;
  hw.PrintLine("---- %s", s);
  for (int i=0; i<8; ++i) {
    for (int j=0; j < 8; ++j) {
      int o = i*8 + j;
      hw.Print("%08x ", emb[o]);
    }
    hw.PrintLine("");
  }
}

extern "C" int cpp_main(void)
{
  //SCB_EnableICache();
  //SCB_EnableDCache();

        // TODO move this earlier
        rust_setup();

	hw.Init();
        initLogging();
	hw.SetAudioBlockSize(4); // number of samples handled per callback
	hw.SetAudioSampleRate(SaiHandle::Config::SampleRate::SAI_48KHZ);

        hw.PrintLine("PatchPtr size %d", get_size());

        dump_allocator("0b");
        thePatchPtr = get_patch(0x10); // 0
        dump_allocator("0a");

        //hw.PrintLine("PatchPtr %p", thePatchPtr);
        //float pf = use_patch(thePatchPtr);
        //hw.PrintLine("PatchPtr foo %f", pf);

        dump_allocator("1b");
        PatchPtr p00 = get_patch(0x20);
        dump_allocator("1a");
        dump_allocator("2b");
        PatchPtr p0 = get_patch(0x30); // 1
        dump_allocator("2a");

        //hw.PrintLine("PatchPtr %p", thePatchPtr);
        float pf2 = use_patch(thePatchPtr);
        //hw.PrintLine("PatchPtr foo %f", pf2);

        dump_allocator("3b");
        PatchPtr p1 = get_patch(0x40); // 2
        dump_allocator("3a");

        //hw.PrintLine("PatchPtr %p", thePatchPtr);
        float pf3 = use_patch(thePatchPtr);
        //hw.PrintLine("PatchPtr foo %f", pf3);

        dump_allocator("4b");
        PatchPtr p2 = get_patch(0x50); // 3
        dump_allocator("4a");

        hw.PrintLine("pz %p %p %p", p0, p1, p2);

        patch_main(thePatchPtr);

        dump_allocator("5b");
        get_patch(0x60); // 4
        dump_allocator("5a");

	hw.StartAudio(AudioCallback);

        patch_main(thePatchPtr);

        while(1) {
          //hw.PrintLine("dl %f %f %f %f %d", inl, inr, outl, outr, frames);
          //hw.PrintLine("audiobufs %p %p %p %p\n", pinl, pinr, poutl, poutr);
          System::Delay(500);
        }
        hw.PrintLine("Looping...");
       while(1) {}
}
