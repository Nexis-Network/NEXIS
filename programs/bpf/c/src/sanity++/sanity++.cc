/**
 * @brief Example C++-based BPF program that prints out the parameters
 * passed to it
 */
#include <nexis_sdk.h>

/**
 * Custom error for when input serialization fails
 */
#define INVALID_INPUT 1

extern uint64_t entrypoint(const uint8_t *input) {
  SolAccountInfo ka[1];
  SolParameters params = (SolParameters) { .ka = ka };

  nzt_log(__FILE__);

  if (!nzt_deserialize(input, &params, NZT_ARRAY_SIZE(ka))) {
    return ERROR_INVALID_ARGUMENT;
  }

  // Log the provided input parameters.  In the case of  the no-op
  // program, no account keys or input data are expected but real
  // programs will have specific requirements so they can do their work.
  nzt_log_params(&params);
  return SUCCESS;
}
