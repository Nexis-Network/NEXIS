/**
 * @brief Example C-based BPF program that prints out the parameters
 * passed to it
 */
#include <nexis_sdk.h>
#include "helper.h"

void helper_function(void) {
  nzt_log(__FILE__);
}
