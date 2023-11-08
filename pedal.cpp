#include "daisy_seed.h"
#include "daisysp.h"

using namespace daisy;
using namespace daisysp;

DaisySeed hw;

extern "C" {
    void rust_function();
    void rust_function2(const float* const* in_ptr, float **out_ptr, size_t len);
}

void foo() {
	rust_function();
}

//static int hey = 0;
float inl, inr, outl, outr;
int haha=0;

//pub const unsafe fn from_raw_parts<'a, T>(data: *const T, len: usize) -> &'a [T]
//typedef const float* const* InputBuffer;
//typedef float** OutputBuffer;
void AudioCallback(AudioHandle::InputBuffer in, AudioHandle::OutputBuffer out, size_t size)
{
  //printf("size_t %d\n", sizeof(size_t));

  /*
  if (hey < 2) {
    //hw.PrintLine("size %d", size);
    //hw.PrintLine("size %d", size);
    //int i = 120;
    //float f = 120.3;
    //hw.PrintLine("Hello World2! %d %d %f %f", 12, i, 12.3, f);
    //hw.PrintLine("Hello World2! %d", 12);
    //hw.PrintLine("size %f", 2.3);
    //hw.PrintLine("callback %f %f %f %f", in[0][0], in[1][0], out[0][0], out[1][0]);
    hey++;
  }
  */

	rust_function();
  rust_function2(in, out, size);

  /*
	for (size_t i = 0; i < size; i++)
	{
		out[0][i] = in[1][i];
		out[1][i] = in[0][i];
	}
        */

  inl = in[0][0];
  inr = in[1][0];
  outl = out[0][0];
  outr = out[1][0];
  haha++;
}


void initLogging() {
  hw.StartLog(true);
  int i = 120;
  float f = 120.3;
  hw.PrintLine("Hello World! %d %d %f %f", 12, i, 12.3, f);
}

int main(void)
{
	hw.Init();
        initLogging();
	foo();
	hw.SetAudioBlockSize(4); // number of samples handled per callback
	hw.SetAudioSampleRate(SaiHandle::Config::SampleRate::SAI_48KHZ);
	hw.StartAudio(AudioCallback);
	while(1) {
          System::Delay(500);
          hw.PrintLine("dl %f %f %f %f %d", inl, inr, outl, outr, haha);
        }
	//while(1) {}
}
