#include "hw.h"
#include "load.h"

extern "C" {
  void rig_process_audio_callback(const float* const* in_ptr, float **out_ptr, size_t len);
  void patch_main();
}

// TODO remove this indirection
void AudioCallback2(AudioHandle::InputBuffer in, AudioHandle::OutputBuffer out, size_t size)
{
  rig_process_audio_callback(in, out, size);
}

extern "C" void cpp_rig_install_callback()
{
	hw.StartAudio(AudioCallback2);
}
