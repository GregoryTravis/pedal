#include "load.h"

#include "constants.h"
#include "spew.h"

CpuLoadMeter cpuLoadMeter;

extern "C" void cpp_load_init() {
  cpuLoadMeter.Init(hw.AudioSampleRate(), hw.AudioBlockSize());
}

extern "C" void cpp_load_before() {
  cpuLoadMeter.OnBlockStart();
}

extern "C" void cpp_load_after() {
  cpuLoadMeter.OnBlockEnd();
}

extern "C" void cpp_load_spew() {
  const float avgLoad = cpuLoadMeter.GetAvgCpuLoad();
  const float maxLoad = cpuLoadMeter.GetMaxCpuLoad();
  const float minLoad = cpuLoadMeter.GetMinCpuLoad();
  if (!PROD) {
    hw.PrintLine("load max: " FLT_FMT3 " min " FLT_FMT3 " avg " FLT_FMT3 "\n", FLT_VAR3(maxLoad * 100.0f), FLT_VAR3(minLoad * 100.0f), FLT_VAR3(avgLoad * 100.0f));
  }
}
