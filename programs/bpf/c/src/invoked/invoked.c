/**
 * @brief Example C-based BPF program that tests cross-program invocations
 */
#include "instruction.h"
#include <nexis_sdk.h>

extern uint64_t entrypoint(const uint8_t *input) {
  nzt_log("Invoked C program");

  SolAccountInfo accounts[4];
  SolParameters params = (SolParameters){.ka = accounts};

  if (!nzt_deserialize(input, &params, 0)) {
    return ERROR_INVALID_ARGUMENT;
  }

  // on entry, return data must not be set
  nzt_assert(nzt_get_return_data(NULL, 0, NULL) == 0);

  if (params.data_len == 0) {
    return SUCCESS;
  }

  switch (params.data[0]) {
  case VERIFY_TRANSLATIONS: {
    nzt_log("verify data translations");

    static const int ARGUMENT_INDEX = 0;
    static const int INVOKED_ARGUMENT_INDEX = 1;
    static const int INVOKED_PROGRAM_INDEX = 2;
    static const int INVOKED_PROGRAM_DUP_INDEX = 3;
    nzt_assert(nzt_deserialize(input, &params, 4));

    SolPubkey bpf_loader_id =
        (SolPubkey){.x = {2,  168, 246, 145, 78,  136, 161, 110, 57,  90, 225,
                          40, 148, 143, 250, 105, 86,  147, 55,  104, 24, 221,
                          71, 67,  82,  33,  243, 198, 0,   0,   0,   0}};

    SolPubkey bpf_loader_deprecated_id =
        (SolPubkey){.x = {2,   168, 246, 145, 78,  136, 161, 107, 189, 35,  149,
                          133, 95,  100, 4,   217, 180, 244, 86,  183, 130, 27,
                          176, 20,  87,  73,  66,  140, 0,   0,   0,   0}};

    for (int i = 0; i < params.data_len; i++) {
      nzt_assert(params.data[i] == i);
    }
    nzt_assert(params.ka_num == 4);

    nzt_assert(*accounts[ARGUMENT_INDEX].lamports == 42);
    nzt_assert(accounts[ARGUMENT_INDEX].data_len == 100);
    nzt_assert(accounts[ARGUMENT_INDEX].is_signer);
    nzt_assert(accounts[ARGUMENT_INDEX].is_writable);
    nzt_assert(accounts[ARGUMENT_INDEX].rent_epoch == 0);
    nzt_assert(!accounts[ARGUMENT_INDEX].executable);
    for (int i = 0; i < accounts[ARGUMENT_INDEX].data_len; i++) {
      nzt_assert(accounts[ARGUMENT_INDEX].data[i] == i);
    }

    nzt_assert(SolPubkey_same(accounts[INVOKED_ARGUMENT_INDEX].owner,
                              accounts[INVOKED_PROGRAM_INDEX].key));
    nzt_assert(*accounts[INVOKED_ARGUMENT_INDEX].lamports == 10);
    nzt_assert(accounts[INVOKED_ARGUMENT_INDEX].data_len == 10);
    nzt_assert(accounts[INVOKED_ARGUMENT_INDEX].is_signer);
    nzt_assert(accounts[INVOKED_ARGUMENT_INDEX].is_writable);
    nzt_assert(accounts[INVOKED_ARGUMENT_INDEX].rent_epoch == 0);
    nzt_assert(!accounts[INVOKED_ARGUMENT_INDEX].executable);

    nzt_assert(
        SolPubkey_same(accounts[INVOKED_PROGRAM_INDEX].key, params.program_id))
        nzt_assert(SolPubkey_same(accounts[INVOKED_PROGRAM_INDEX].owner,
                                  &bpf_loader_id));
    nzt_assert(!accounts[INVOKED_PROGRAM_INDEX].is_signer);
    nzt_assert(!accounts[INVOKED_PROGRAM_INDEX].is_writable);
    nzt_assert(accounts[INVOKED_PROGRAM_INDEX].rent_epoch == 0);
    nzt_assert(accounts[INVOKED_PROGRAM_INDEX].executable);

    nzt_assert(SolPubkey_same(accounts[INVOKED_PROGRAM_INDEX].key,
                              accounts[INVOKED_PROGRAM_DUP_INDEX].key));
    nzt_assert(SolPubkey_same(accounts[INVOKED_PROGRAM_INDEX].owner,
                              accounts[INVOKED_PROGRAM_DUP_INDEX].owner));
    nzt_assert(*accounts[INVOKED_PROGRAM_INDEX].lamports ==
               *accounts[INVOKED_PROGRAM_DUP_INDEX].lamports);
    nzt_assert(accounts[INVOKED_PROGRAM_INDEX].is_signer ==
               accounts[INVOKED_PROGRAM_DUP_INDEX].is_signer);
    nzt_assert(accounts[INVOKED_PROGRAM_INDEX].is_writable ==
               accounts[INVOKED_PROGRAM_DUP_INDEX].is_writable);
    nzt_assert(accounts[INVOKED_PROGRAM_INDEX].rent_epoch ==
               accounts[INVOKED_PROGRAM_DUP_INDEX].rent_epoch);
    nzt_assert(accounts[INVOKED_PROGRAM_INDEX].executable ==
               accounts[INVOKED_PROGRAM_DUP_INDEX].executable);
    break;
  }
  case RETURN_OK: {
    nzt_log("return Ok");
    return SUCCESS;
  }
  case SET_RETURN_DATA: {
    nzt_set_return_data((const uint8_t*)RETURN_DATA_VAL, sizeof(RETURN_DATA_VAL));
    nzt_log("set return data");
    nzt_assert(nzt_get_return_data(NULL, 0, NULL) == sizeof(RETURN_DATA_VAL));
    return SUCCESS;
  }
  case RETURN_ERROR: {
    nzt_log("return error");
    return 42;
  }
  case DERIVED_SIGNERS: {
    nzt_log("verify derived signers");
    static const int INVOKED_PROGRAM_INDEX = 0;
    static const int DERIVED_KEY1_INDEX = 1;
    static const int DERIVED_KEY2_INDEX = 2;
    static const int DERIVED_KEY3_INDEX = 3;
    nzt_assert(nzt_deserialize(input, &params, 4));

    nzt_assert(accounts[DERIVED_KEY1_INDEX].is_signer);
    nzt_assert(!accounts[DERIVED_KEY2_INDEX].is_signer);
    nzt_assert(!accounts[DERIVED_KEY2_INDEX].is_signer);

    uint8_t bump_seed2 = params.data[1];
    uint8_t bump_seed3 = params.data[2];

    SolAccountMeta arguments[] = {
        {accounts[DERIVED_KEY1_INDEX].key, true, false},
        {accounts[DERIVED_KEY2_INDEX].key, true, true},
        {accounts[DERIVED_KEY3_INDEX].key, false, true}};
    uint8_t data[] = {VERIFY_NESTED_SIGNERS};
    const SolInstruction instruction = {accounts[INVOKED_PROGRAM_INDEX].key,
                                        arguments, NZT_ARRAY_SIZE(arguments),
                                        data, NZT_ARRAY_SIZE(data)};
    uint8_t seed1[] = {'L', 'i', 'l', '\''};
    uint8_t seed2[] = {'B', 'i', 't', 's'};
    const SolSignerSeed seeds1[] = {{seed1, NZT_ARRAY_SIZE(seed1)},
                                    {seed2, NZT_ARRAY_SIZE(seed2)},
                                    {&bump_seed2, 1}};
    const SolSignerSeed seeds2[] = {
        {(uint8_t *)accounts[DERIVED_KEY2_INDEX].key, SIZE_PUBKEY},
        {&bump_seed3, 1}};
    const SolSignerSeeds signers_seeds[] = {{seeds1, NZT_ARRAY_SIZE(seeds1)},
                                            {seeds2, NZT_ARRAY_SIZE(seeds2)}};

    nzt_assert(SUCCESS == nzt_invoke_signed(&instruction, accounts,
                                            params.ka_num, signers_seeds,
                                            NZT_ARRAY_SIZE(signers_seeds)));

    break;
  }

  case VERIFY_NESTED_SIGNERS: {
    nzt_log("verify derived nested signers");
    static const int DERIVED_KEY1_INDEX = 0;
    static const int DERIVED_KEY2_INDEX = 1;
    static const int DERIVED_KEY3_INDEX = 2;
    nzt_assert(nzt_deserialize(input, &params, 3));

    nzt_assert(!accounts[DERIVED_KEY1_INDEX].is_signer);
    nzt_assert(accounts[DERIVED_KEY2_INDEX].is_signer);
    nzt_assert(accounts[DERIVED_KEY2_INDEX].is_signer);

    break;
  }

  case VERIFY_WRITER: {
    nzt_log("verify writable");
    static const int ARGUMENT_INDEX = 0;
    nzt_assert(nzt_deserialize(input, &params, 1));

    nzt_assert(accounts[ARGUMENT_INDEX].is_writable);
    break;
  }

  case VERIFY_PRIVILEGE_ESCALATION: {
    nzt_log("Verify privilege escalation");
    break;
  }

  case VERIFY_PRIVILEGE_DEESCALATION: {
    nzt_log("verify privilege deescalation");
    static const int INVOKED_ARGUMENT_INDEX = 0;
    nzt_assert(false == accounts[INVOKED_ARGUMENT_INDEX].is_signer);
    nzt_assert(false == accounts[INVOKED_ARGUMENT_INDEX].is_writable);
    break;
  }
  case VERIFY_PRIVILEGE_DEESCALATION_ESCALATION_SIGNER: {
    nzt_log("verify privilege deescalation escalation signer");
    static const int INVOKED_PROGRAM_INDEX = 0;
    static const int INVOKED_ARGUMENT_INDEX = 1;
    nzt_assert(nzt_deserialize(input, &params, 2));

    nzt_assert(false == accounts[INVOKED_ARGUMENT_INDEX].is_signer);
    nzt_assert(false == accounts[INVOKED_ARGUMENT_INDEX].is_writable);
    SolAccountMeta arguments[] = {
        {accounts[INVOKED_ARGUMENT_INDEX].key, true, false}};
    uint8_t data[] = {VERIFY_PRIVILEGE_ESCALATION};
    const SolInstruction instruction = {accounts[INVOKED_PROGRAM_INDEX].key,
                                        arguments, NZT_ARRAY_SIZE(arguments),
                                        data, NZT_ARRAY_SIZE(data)};
    nzt_assert(SUCCESS ==
               nzt_invoke(&instruction, accounts, NZT_ARRAY_SIZE(accounts)));
    break;
  }

  case VERIFY_PRIVILEGE_DEESCALATION_ESCALATION_WRITABLE: {
    nzt_log("verify privilege deescalation escalation writable");
    static const int INVOKED_PROGRAM_INDEX = 0;
    static const int INVOKED_ARGUMENT_INDEX = 1;
    nzt_assert(nzt_deserialize(input, &params, 2));

    nzt_assert(false == accounts[INVOKED_ARGUMENT_INDEX].is_signer);
    nzt_assert(false == accounts[INVOKED_ARGUMENT_INDEX].is_writable);
    SolAccountMeta arguments[] = {
        {accounts[INVOKED_ARGUMENT_INDEX].key, false, true}};
    uint8_t data[] = {VERIFY_PRIVILEGE_ESCALATION};
    const SolInstruction instruction = {accounts[INVOKED_PROGRAM_INDEX].key,
                                        arguments, NZT_ARRAY_SIZE(arguments),
                                        data, NZT_ARRAY_SIZE(data)};
    nzt_assert(SUCCESS ==
               nzt_invoke(&instruction, accounts, NZT_ARRAY_SIZE(accounts)));
    break;
  }

  case NESTED_INVOKE: {
    nzt_log("invoke");

    static const int INVOKED_ARGUMENT_INDEX = 0;
    static const int ARGUMENT_INDEX = 1;
    static const int INVOKED_PROGRAM_INDEX = 2;

    if (!nzt_deserialize(input, &params, 3)) {
      nzt_assert(nzt_deserialize(input, &params, 2));
    }

    nzt_assert(nzt_deserialize(input, &params, 2));

    nzt_assert(accounts[INVOKED_ARGUMENT_INDEX].is_signer);
    nzt_assert(accounts[ARGUMENT_INDEX].is_signer);

    *accounts[INVOKED_ARGUMENT_INDEX].lamports -= 1;
    *accounts[ARGUMENT_INDEX].lamports += 1;

    uint8_t remaining_invokes = params.data[1];
    if (remaining_invokes > 1) {
      nzt_log("Invoke again");
      SolAccountMeta arguments[] = {
          {accounts[INVOKED_ARGUMENT_INDEX].key, true, true},
          {accounts[ARGUMENT_INDEX].key, true, true},
          {accounts[INVOKED_PROGRAM_INDEX].key, false, false}};
      uint8_t data[] = {NESTED_INVOKE, remaining_invokes - 1};
      const SolInstruction instruction = {accounts[INVOKED_PROGRAM_INDEX].key,
                                          arguments, NZT_ARRAY_SIZE(arguments),
                                          data, NZT_ARRAY_SIZE(data)};
      nzt_assert(SUCCESS == nzt_invoke(&instruction, accounts, params.ka_num));
    } else {
      nzt_log("Last invoked");
      for (int i = 0; i < accounts[INVOKED_ARGUMENT_INDEX].data_len; i++) {
        accounts[INVOKED_ARGUMENT_INDEX].data[i] = i;
      }
    }
    break;
  }

  case WRITE_ACCOUNT: {
    nzt_log("write account");
    static const int INVOKED_ARGUMENT_INDEX = 0;
    nzt_assert(nzt_deserialize(input, &params, 1));

    for (int i = 0; i < params.data[1]; i++) {
      accounts[INVOKED_ARGUMENT_INDEX].data[i] = params.data[1];
    }
    break;
  }

  default:
    return ERROR_INVALID_INSTRUCTION_DATA;
  }
  return SUCCESS;
}
