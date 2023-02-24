use bollard::Docker;

use crate::config::{generate_demangle_rust_flags, BuildConfig};

#[derive(Debug, Clone)]
pub struct BuildState {
    pub docker: Docker,
    pub clean: bool,
    pub verbose: bool,
}

pub fn generate_build_env(config: &BuildConfig) -> Vec<String> {
    let env = vec![
        "RUSTFLAGS=".to_string() + config.rust_flags.as_str(),
        "LD_SCRIPT_PATH=".to_string() + config.ld_script_path.as_str(),
        "KERNEL_SYMBOLS_DEMANGLED_RS=".to_string()
            + config.kernel_symbols_elf.as_str()
            + "_demangled.rs",
    ];

    return env;
}

pub fn generate_demangle_build_env(config: &BuildConfig, address: String) -> Vec<String> {
    let env = vec![
        "RUSTFLAGS=".to_string() + generate_demangle_rust_flags(address).as_str(),
        "LD_SCRIPT_PATH=".to_string() + config.ld_script_path.as_str(),
        "KERNEL_SYMBOLS_DEMANGLED_RS=".to_string()
            + config.kernel_symbols_elf.as_str()
            + "_demangled.rs",
    ];

    return env;
}
