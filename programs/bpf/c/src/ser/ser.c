/**
 * @brief Example C-based BPF sanity rogram that prints out the parameters
 * passed to it
 */
#include <nexis_sdk.h>

extern uint64_t entrypoint(const uint8_t *input) {
  SolAccountInfo ka[2];
  SolParameters params = (SolParameters){.ka = ka};

  nzt_log(__FILE__);

  if (!nzt_deserialize(input, &params, NZT_ARRAY_SIZE(ka))) {
    return ERROR_INVALID_ARGUMENT;
  }

  char ka_data[] = {1, 2, 3};
  SolPubkey ka_owner;
  nzt_memset(ka_owner.x, 0, SIZE_PUBKEY); // set to system program

  nzt_assert(params.ka_num == 2);
  for (int i = 0; i < 2; i++) {
    nzt_assert(*params.ka[i].lamports == 42);
    nzt_assert(!nzt_memcmp(params.ka[i].data, ka_data, 4));
    nzt_assert(SolPubkey_same(params.ka[i].owner, &ka_owner));
    nzt_assert(params.ka[i].is_signer == false);
    nzt_assert(params.ka[i].is_writable == false);
    nzt_assert(params.ka[i].executable == false);
  }

  char data[] = {4, 5, 6, 7};
  nzt_assert(params.data_len = 4);
  nzt_assert(!nzt_memcmp(params.data, data, 4));

  return SUCCESS;
}
