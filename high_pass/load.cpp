#include "load.h"

#include "spew.h"

#if CPU_LOAD
CpuLoadMeter cpuLoadMeter;

void load_init() {
  cpuLoadMeter.Init(hw.AudioSampleRate(), hw.AudioBlockSize());
}

void load_before() {
  cpuLoadMeter.OnBlockStart();
}

void load_after() {
  cpuLoadMeter.OnBlockEnd();
}

extern "C" void load_spew() {
  const float avgLoad = cpuLoadMeter.GetAvgCpuLoad();
  const float maxLoad = cpuLoadMeter.GetMaxCpuLoad();
  const float minLoad = cpuLoadMeter.GetMinCpuLoad();
  hw.PrintLine("load max: " FLT_FMT3 " min " FLT_FMT3 " avg " FLT_FMT3 "\n", FLT_VAR3(maxLoad * 100.0f), FLT_VAR3(minLoad * 100.0f), FLT_VAR3(avgLoad * 100.0f));
}

#else

#define load_init ((void)0)
#define load_before ((void)0)
#define load_after ((void)0)
#define load_spew ((void)0)

#endif
