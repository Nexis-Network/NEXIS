nexis_sdk::declare_builtin!(
    nexis_sdk::bpf_loader_deprecated::ID,
    nexis_bpf_loader_deprecated_program,
    nexis_bpf_loader_program::process_instruction,
    deprecated::id
);
