nexis_sdk::declare_builtin!(
    nexis_sdk::bpf_loader_upgradeable::ID,
    nexis_bpf_loader_upgradeable_program,
    nexis_bpf_loader_program::process_instruction,
    upgradeable::id
);
