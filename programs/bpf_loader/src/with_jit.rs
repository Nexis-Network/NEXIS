nexis_sdk::declare_builtin!(
    nexis_sdk::bpf_loader::ID,
    nexis_bpf_loader_program_with_jit,
    nexis_bpf_loader_program::process_instruction_jit
);
