#include "hw.h"

extern "C" void cpp_hw_Init() {
	hw.Init();
}

extern "C" void cpp_hw_StartLog(bool b) {
  hw.StartLog(b);
}
