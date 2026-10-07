/* Unmodified FP_TYPE=float/double math helpers and table declarations. */
#include <stdint.h>
#include <string.h>
#include "math-funcs.h"
#ifdef _WIN32
#include <fcntl.h>
#include <io.h>
#endif

static void integer(uint32_t value) {
  unsigned char bytes[4] = { value, value >> 8, value >> 16, value >> 24 };
  fwrite(bytes, 1, 4, stdout);
}

static FP_TYPE neighbor(FP_TYPE value, double direction) {
  return sizeof(FP_TYPE) == sizeof(float)
    ? (FP_TYPE)nextafterf((float)value, (float)direction)
    : (FP_TYPE)nextafter((double)value, direction);
}

static void record(uint32_t kind, FP_TYPE a, FP_TYPE b) {
  FP_TYPE output = kind == 0 ? lrh_lse(a, b) : kind == 1 ? lrh_negexp(a) : lrh_exp(a);
  integer(kind);
  fwrite(&a, sizeof a, 1, stdout);
  fwrite(&b, sizeof b, 1, stdout);
  fwrite(&output, sizeof output, 1, stdout);
}

int main(void) {
#ifdef _WIN32
  _setmode(_fileno(stdout), _O_BINARY);
#endif
  fwrite("NUM1", 1, 4, stdout);
  integer(sizeof(FP_TYPE));
  integer(7000);
  fwrite(lrh_lse_table, sizeof(FP_TYPE), 7000, stdout);
  fwrite(lrh_exp_table, sizeof(FP_TYPE), 7000, stdout);
  integer(42008);
  for(int i = 0; i < 7000; ++i) {
    FP_TYPE x = (FP_TYPE)(i * 0.001);
    record(0, 0, -neighbor(x, -INFINITY));
    record(0, 0, -x);
    record(0, 0, -neighbor(x, INFINITY));
    x = (FP_TYPE)(-i * 0.002);
    record(1, neighbor(x, -INFINITY), 0);
    record(1, x, 0);
    record(1, neighbor(x, INFINITY), 0);
  }
  const double inputs[8] = {-14, -13.9, -0.125, 0, 0.25, 1, 40, 80};
  for(int i = 0; i < 8; ++i) record(2, (FP_TYPE)inputs[i], 0);
  return ferror(stdout) ? 1 : 0;
}
