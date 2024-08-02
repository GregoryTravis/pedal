#include<array> 
#include <vector>

#include "hw.h"

static std::vector<AnalogControl> knobs;

extern "C" void cpp_knob_init() {
    Pin knobPins[] = {seed::D15, seed::D16, seed::D17, seed::D18, seed::D19, seed::D20};
    const int count = std::end(knobPins) - std::begin(knobPins);
    assert(count == 6);

    AdcChannelConfig cfg[count];

    // Init with Single Pins
    for (int i = 0; i < count; i++)
    {
        cfg[i].InitSingle(knobPins[i]);
    }

    hw.adc.Init(cfg, count);

    auto audioCallbackRate = hw.AudioCallbackRate();

    // Setup the Knobs
    for(int i = 0; i < count; i++)
    {
        AnalogControl myKnob;
        myKnob.Init(hw.adc.GetPtr(i), audioCallbackRate);
        knobs.push_back(myKnob);
    }

    for(uint i = 0; i < knobs.size(); i++)
    {
        // redundant?
        knobs[i].SetSampleRate(audioCallbackRate);
    }
}

extern "C" void cpp_knob_process() {
    if (!knobs.empty())
    {
        for(uint i = 0; i < knobs.size(); i++)
        {
            knobs[i].Process();
        }
    }
}

extern "C" float cpp_knob_get_value(size_t knobID) {
    assert(!knobs.empty() && knobID >= 0 && knobID < knobs.size());

    if (!knobs.empty() && knobID >= 0 && knobID < knobs.size())
    {
        return knobs[knobID].Value();
    }

    return 0.0f;
}
