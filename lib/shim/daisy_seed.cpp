#include "hw.h"

#include "constants.h"
#include "load.h"

extern "C" size_t cpp_hw_get_size_t_size() {
  return sizeof(size_t);
}

void sanity_check() {
  assert(sizeof(float) == 4);
}

extern "C" void cpp_hw_init(bool b, size_t block_size) {
  sanity_check();

	hw.Init();
  if (!PROD) {
    hw.StartLog(b);
  }
  hw.SetAudioBlockSize(block_size);
  assert(hw.audio_handle.GetChannels() == 2);
}

GPIO audioBypassTrigger;
GPIO audioMuteTrigger;

static Pin ledPins[] = {seed::D22, seed::D23};
static int numLeds = sizeof(ledPins) / sizeof(ledPins[0]);
static bool ledInitted[] = { false, false };
static Led leds[2];
extern "C" void cpp_hw_set_led(int index, float brightness) {
  if (index < 0 || index >= numLeds) {
    return;
  }

  if (!ledInitted[index]) {
    leds[index].Init(ledPins[index], false, hw.AudioCallbackRate());
    leds[index].SetSampleRate(hw.AudioCallbackRate()); // Necessary?
    ledInitted[index] = true;
  }
  leds[index].Set(brightness);
  leds[index].Update(); // Necessary?
}

extern "C" void cpp_hw_kshep_init() {
  Pin relayPin = seed::D1;
  Pin mutePin = seed::D12;
  audioBypassTrigger.Init(relayPin, GPIO::Mode::OUTPUT);
  audioMuteTrigger.Init(mutePin, GPIO::Mode::OUTPUT);

  bool m_audioBypass = false;
  audioBypassTrigger.Write(!m_audioBypass);
  bool m_audioMute = false;
  audioMuteTrigger.Write(m_audioMute);

  hw.adc.Start();
}

extern "C" void cpp_hw_delay(uint32_t delay_ms) {
  System::Delay(delay_ms);
}

// Returns a relative time in milliseconds.
extern "C" uint32_t cpp_relative_time_ms() {
  return hw.system.GetNow();
}
