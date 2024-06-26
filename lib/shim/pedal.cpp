#include <assert.h>

#include "daisy_seed.h"

#include "hw.h"
#include "load.h"
#include "spew.h"

#define SPEED_TEST 1

using namespace daisy;

#define TEST_CPP 0

#if TEST_CPP
#include "patch.h"
#endif

extern "C" {
  void rust_process_audio_stub(const float* const* in_ptr, float **out_ptr, size_t len);
  void patch_main();
  int PEDAL_MAIN();
  void rust_speed_test_init();
  float rust_f32_dot();
  float rust_f32_circsum();
}

// (libDaisy/src/hid/audio.h)
// Non-Interleaving output buffer
// Arranged by float[chn][sample] 
// Left 0, Right 1
// The mono pedal is right only
// typedef const float* const* InputBuffer;
// typedef float** OutputBuffer;
void AudioCallback(AudioHandle::InputBuffer in, AudioHandle::OutputBuffer out, size_t size)
{
  cpp_load_before();
  rust_process_audio_stub(in, out, size);
  cpp_load_after();
}

void initLogging() {
  hw.StartLog(true);
  hw.PrintLine("Pedal!");
}

extern "C" void UnsafeDelay(uint32_t delay_ms) {
  System::Delay(delay_ms);
}

GPIO audioBypassTrigger;
GPIO audioMuteTrigger;

extern "C" int cpp_main(void)
{
	hw.Init();
  initLogging();
	hw.SetAudioBlockSize(48); // number of samples handled per callback
	hw.SetAudioSampleRate(SaiHandle::Config::SampleRate::SAI_48KHZ);

  assert(hw.audio_handle.GetChannels() == 2);

  Pin relayPin = seed::D1;
  Pin mutePin = seed::D12;
  audioBypassTrigger.Init(relayPin, GPIO::Mode::OUTPUT);
  audioMuteTrigger.Init(mutePin, GPIO::Mode::OUTPUT);

  bool m_audioBypass = false;
  audioBypassTrigger.Write(!m_audioBypass);
  bool m_audioMute = false;
  audioMuteTrigger.Write(m_audioMute);

  cpp_load_init();

#if TEST_CPP
	hw.StartAudio(CppVibratoAudioCallback);
#else
	hw.StartAudio(AudioCallback);
#endif

  patch_main();
  while(1) {} // Just in case we fall through
}

#if SPEED_TEST

#include "speed_test.h"

static long ticks = 0;
void speed_test_callback(AudioHandle::InputBuffer _in, AudioHandle::OutputBuffer _out, size_t _size)
{
  ticks++;
}

#define TEST_FOR 100

void speed_test(const char *test_name, float (*f)()) {
  long start = ticks;
  long end = ticks + TEST_FOR;
  long num_calls = 0;
  bool stop_early = false;
  int stop_early_count = 1000000;
  float fr;
  while (ticks < end && (!stop_early || num_calls < stop_early_count)) {
    fr = f();
    num_calls++;
  }
  long num_ticks = end - start;
  hw.PrintLine("%s %d %d %f", test_name, num_ticks, num_calls, fr);
}

void speed_test_main() {
  cpp_speed_test_init();
  rust_speed_test_init();

  speed_test("cpp_f32_circsum", &cpp_f32_circsum);
  speed_test("rust_f32_circsum", &rust_f32_circsum);
}

int xmain() {
	hw.Init();
  initLogging();
	hw.SetAudioBlockSize(48); // number of samples handled per callback
	hw.SetAudioSampleRate(SaiHandle::Config::SampleRate::SAI_48KHZ);

  assert(hw.audio_handle.GetChannels() == 2);

	hw.StartAudio(&speed_test_callback);

  hw.PrintLine("float %d\n", sizeof(float));

  speed_test_main();

  return 0;
}

#else

int main() {
#if TEST_CPP
  patch_setup();
#endif
  PEDAL_MAIN();
}

#endif  // if SPEED_TEST
