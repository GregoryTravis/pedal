#include "constants.h"
#include "hw.h"

static Pin switchPins[] = {seed::D6, seed::D5};
static size_t numSwitches = sizeof(switchPins) / sizeof(switchPins[0]);
static Switch switches[2];
static bool initted = false;

extern "C" void cpp_switch_init() {
  assert(KSHEP);

  for (size_t i = 0; i < numSwitches; ++i) {
    switches[i].Init(switchPins[i], 0.0f, Switch::Type::TYPE_TOGGLE, Switch::Polarity::POLARITY_INVERTED, Switch::Pull::PULL_UP);
  }

  initted = true;
}

extern "C" void cpp_switch_process() {
  for (size_t i = 0; i < numSwitches; ++i) {
    switches[i].Debounce();
  }
}

extern "C" bool cpp_switch_get_value(size_t n) {
  if (!initted || n < 0 || n >= numSwitches) {
    return false;
  }

  return switches[n].Pressed();
}
