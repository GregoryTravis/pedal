#include "hw.h"
#include "load.h"

extern "C" {
  void rust_process_audio_stub(const float* const* in_ptr, float **out_ptr, size_t len);
  void patch_main();
}

void AudioCallback2(AudioHandle::InputBuffer in, AudioHandle::OutputBuffer out, size_t size)
{
  cpp_load_before();
  rust_process_audio_stub(in, out, size);
  cpp_load_after();
}

extern "C" void cpp_rig_install_callback()
{
	hw.StartAudio(AudioCallback2);
  //patch_main();
  //while(1) {} // Just in case we fall through
}

