#include "hw.h"
#include "load.h"

extern "C" size_t cpp_hw_get_size_t_size() {
  return sizeof(size_t);
}

extern "C" void cpp_hw_init(bool b, size_t block_size) {
	hw.Init();
  hw.StartLog(b);
  hw.SetAudioBlockSize(block_size);
  assert(hw.audio_handle.GetChannels() == 2);
}

GPIO audioBypassTrigger;
GPIO audioMuteTrigger;

extern "C" void cpp_hw_kshep_init() {
  Pin relayPin = seed::D1;
  Pin mutePin = seed::D12;
  audioBypassTrigger.Init(relayPin, GPIO::Mode::OUTPUT);
  audioMuteTrigger.Init(mutePin, GPIO::Mode::OUTPUT);

  bool m_audioBypass = false;
  audioBypassTrigger.Write(!m_audioBypass);
  bool m_audioMute = false;
  audioMuteTrigger.Write(m_audioMute);
}

extern "C" void cpp_hw_delay(uint32_t delay_ms) {
  System::Delay(delay_ms);
}
