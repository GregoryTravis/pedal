#ifndef _constants_h_
#define _constants_h_

#include "daisy_seed.h"

// TODO: generate these from a shared config.
#define SAMPLE_RATE 48000 // SaiHandle::Config::SampleRate::SAI_48KHZ
#define BLOCK_SIZE 48
#define KSHEP true
#define PROD false

// I think the 128 makes room for the rest of the SDRAM struct.
#define SDRAM_SIZE_BYTES ((64 * 1024 * 1024) - 128)
#define SDRAM_SIZE_F32 (SDRAM_SIZE_BYTES / sizeof(float))

#define FFT_SIZE 2048

#endif // _constants_h_
