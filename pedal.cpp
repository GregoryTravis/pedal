#include "daisy_seed.h"
#include "daisysp.h"

using namespace daisy;
using namespace daisysp;

DaisySeed hw;

extern "C" {
    void rust_function();
    void rust_function2(const float *const in_ptr, float *out_ptr, size_t len);
}

void foo() {
	rust_function();
}

//pub const unsafe fn from_raw_parts<'a, T>(data: *const T, len: usize) -> &'a [T]
void AudioCallback(AudioHandle::InputBuffer in, AudioHandle::OutputBuffer out, size_t size)
{
  printf("size_t %d\n", sizeof(size_t));
  rust_function2((const float*) in, (float*) out, size);
  /*
	for (size_t i = 0; i < size; i++)
	{
		out[0][i] = in[0][i];
		out[1][i] = in[1][i];
	}
        */
}

int main(void)
{
	hw.Init();
	foo();
	hw.SetAudioBlockSize(4); // number of samples handled per callback
	hw.SetAudioSampleRate(SaiHandle::Config::SampleRate::SAI_48KHZ);
	hw.StartAudio(AudioCallback);
	while(1) {}
}
