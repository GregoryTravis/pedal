#include "hw.h"

extern "C" size_t cpp_hw_get_size_t_size() {
  return sizeof(size_t);
}

extern "C" void cpp_hw_init(bool b, size_t block_size) {
	hw.Init();
  hw.StartLog(b);
  hw.SetAudioBlockSize(block_size);
}
