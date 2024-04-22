#include "vibrato.h"

#include "daisy_seed.h"

using namespace daisy;

void CppVibratoAudioCallback(AudioHandle::InputBuffer in, AudioHandle::OutputBuffer out, size_t size);
void patch_setup();
