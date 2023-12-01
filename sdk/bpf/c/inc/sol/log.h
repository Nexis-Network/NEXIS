#pragma once
/**
 * @brief Nexis logging utilities
 */

#include <sol/types.h>
#include <sol/string.h>
#include <sol/entrypoint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Prints a string to stdout
 */
void nzt_log_(const char *, uint64_t);
#define nzt_log(message) nzt_log_(message, nzt_strlen(message))

/**
 * Prints a 64 bit values represented in hexadecimal to stdout
 */
void nzt_log_64_(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
#define nzt_log_64 nzt_log_64_

/**
 * Prints the current compute unit consumption to stdout
 */
void nzt_log_compute_units_();
#define nzt_log_compute_units() nzt_log_compute_units_()

/**
 * Prints the hexadecimal representation of an array
 *
 * @param array The array to print
 */
static void nzt_log_array(const uint8_t *array, int len) {
  for (int j = 0; j < len; j++) {
    nzt_log_64(0, 0, 0, j, array[j]);
  }
}

/**
 * Print the base64 representation of some arrays.
 */
void nzt_log_data(SolBytes *fields, uint64_t fields_len);

/**
 * Prints the program's input parameters
 *
 * @param params Pointer to a SolParameters structure
 */
static void nzt_log_params(const SolParameters *params) {
  nzt_log("- Program identifier:");
  nzt_log_pubkey(params->program_id);

  nzt_log("- Number of KeyedAccounts");
  nzt_log_64(0, 0, 0, 0, params->ka_num);
  for (int i = 0; i < params->ka_num; i++) {
    nzt_log("  - Is signer");
    nzt_log_64(0, 0, 0, 0, params->ka[i].is_signer);
    nzt_log("  - Is writable");
    nzt_log_64(0, 0, 0, 0, params->ka[i].is_writable);
    nzt_log("  - Key");
    nzt_log_pubkey(params->ka[i].key);
    nzt_log("  - Lamports");
    nzt_log_64(0, 0, 0, 0, *params->ka[i].lamports);
    nzt_log("  - data");
    nzt_log_array(params->ka[i].data, params->ka[i].data_len);
    nzt_log("  - Owner");
    nzt_log_pubkey(params->ka[i].owner);
    nzt_log("  - Executable");
    nzt_log_64(0, 0, 0, 0, params->ka[i].executable);
    nzt_log("  - Rent Epoch");
    nzt_log_64(0, 0, 0, 0, params->ka[i].rent_epoch);
  }
  nzt_log("- Instruction data\0");
  nzt_log_array(params->data, params->data_len);
}

#ifdef NZT_TEST
/**
 * Stub functions when building tests
 */
#include <stdio.h>

void nzt_log_(const char *s, uint64_t len) {
  printf("Program log: %s\n", s);
}
void nzt_log_64(uint64_t arg1, uint64_t arg2, uint64_t arg3, uint64_t arg4, uint64_t arg5) {
  printf("Program log: %llu, %llu, %llu, %llu, %llu\n", arg1, arg2, arg3, arg4, arg5);
}

void nzt_log_compute_units_() {
  printf("Program consumption: __ units remaining\n");
}
#endif

#ifdef __cplusplus
}
#endif

/**@}*/
