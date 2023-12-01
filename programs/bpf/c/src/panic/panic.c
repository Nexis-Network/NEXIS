/**
 * @brief Example C-based BPF program that prints out the parameters
 * passed to it
 */
#include <nexis_sdk.h>

extern uint64_t entrypoint(const uint8_t *input) {
  nzt_panic();
  return SUCCESS;
}
