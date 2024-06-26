#include "hw.h"

extern "C" void cpp_hw_Init() {
	hw.Init();
}

extern "C" void cpp_hw_StartLog(bool b) {
  hw.StartLog(b);
}

extern "C" size_t cpp_hw_get_size_t_size() {
  return sizeof(size_t);
}

extern "C" void cpp_hw_SetAudioBlockSize(size_t block_size) {
  hw.SetAudioBlockSize(block_size);
}
