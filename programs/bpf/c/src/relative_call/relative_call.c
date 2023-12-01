/**
 * @brief test program that generates BPF PC relative call instructions
 */
#include <nexis_sdk.h>

void __attribute__ ((noinline)) helper() {
  nzt_log(__func__);
}

extern uint64_t entrypoint(const uint8_t *input) {
  nzt_log(__func__);
  helper();
  return SUCCESS;
}
