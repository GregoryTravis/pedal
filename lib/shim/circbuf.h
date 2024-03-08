#include <assert.h>

class CircBuf {
  public:

  CircBuf(size_t size) : size(size), start_index(0) {
    values = new float[size];
    assert(values != nullptr);
    for (size_t i = 0; i < size; ++i) {
      values[i] = 0.0;
    }
  }

  // Add to "end"
  void push(float x) {
    values[start_index] = x;
    start_index = (start_index + 1) % size;
    assert(start_index < size);
  }

  float get(size_t i) {
    assert(i < size);
    return values[(start_index + i) % size];
  }

  private:

  size_t size;
  size_t start_index;
  float *values;
};

