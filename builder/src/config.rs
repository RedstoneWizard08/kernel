pub const DEFAULT_FINAL_BINARY: &'static str = "kernel8.img";
pub const DEFAULT_CPU: &'static str = "cortex-a72";
pub const DEFAULT_LD_SCRIPT_PATH: &'static str = "kernel/src/bsp/aarch64";
pub const DEFAULT_KERNEL_LINKER_SCRIPT: &'static str = "kernel.ld";
pub const DEFAULT_KERNEL_SYMBOLS_LINKER_SCRIPT: &'static str = "kernel_symbols/kernel_symbols.ld";
pub const DEFAULT_TARGET: &'static str = "aarch64-unknown-none-softfloat";
pub const DEFAULT_KERNEL_MANIFEST: &'static str = "kernel/Cargo.toml";
pub const DEFAULT_KERNEL_SYMBOLS_MANIFEST: &'static str = "kernel_symbols/Cargo.toml";
pub const DEFAULT_IMAGE: &'static str = "docker.io/redstonewizard/kernel-builder:latest";
pub const DEFAULT_BSP: &'static str = "rpi4";

pub const DEFAULT_KERNEL_SYMBOLS_ELF: &'static str =
    "target/aarch64-unknown-none-softfloat/release/kernel_symbols";

pub const DEFAULT_RAW_KERNEL_ELF: &'static str =
    "target/aarch64-unknown-none-softfloat/release/kernel";

pub const DEFAULT_ELF_TTABLES: &'static str =
    "target/aarch64-unknown-none-softfloat/release/kernel+ttables";

pub const DEFAULT_ELF_TTABLES_SYMBOLS: &'static str =
    "target/aarch64-unknown-none-softfloat/release/kernel+ttables+symbols";

pub fn generate_rust_flags() -> String {
    let mut flags: Vec<String> = Vec::new();

    flags.push("-C".to_string());
    flags.push("target-cpu=".to_string() + DEFAULT_CPU);
    flags.push("-C".to_string());
    flags.push("force-frame-pointers".to_string());
    flags.push("-C".to_string());
    flags.push("link-arg=--library-path=/work/".to_string() + DEFAULT_LD_SCRIPT_PATH);
    flags.push("-C".to_string());
    flags.push("link-arg=--script=".to_string() + DEFAULT_KERNEL_LINKER_SCRIPT);

    return flags.join(" ");
}

pub fn generate_demangle_rust_flags(address: String) -> String {
    let mut flags: Vec<String> = Vec::new();

    flags.push("-C".to_string());
    flags.push("link-arg=--script=".to_string() + DEFAULT_KERNEL_SYMBOLS_LINKER_SCRIPT);
    flags.push("-C".to_string());
    flags.push("link-arg=--section-start=.rodata=".to_string() + address.as_str());

    return flags.join(" ");
}

pub struct BuildConfig {
    pub rust_flags: String,
    pub ld_script_path: String,
    pub target: String,
    pub kernel_symbols_elf: String,
    pub kernel_manifest: String,
    pub image: String,
    pub raw_kernel_elf: String,
    pub elf_ttables: String,
    pub elf_ttables_symbols: String,
    pub bsp: String,
    pub kernel_symbols_manifest: String,
    pub final_binary: String,
}

impl Default for BuildConfig {
    fn default() -> Self {
        return Self {
            rust_flags: generate_rust_flags(),
            ld_script_path: DEFAULT_LD_SCRIPT_PATH.to_string(),
            target: DEFAULT_TARGET.to_string(),
            kernel_symbols_elf: DEFAULT_KERNEL_SYMBOLS_ELF.to_string(),
            kernel_manifest: DEFAULT_KERNEL_MANIFEST.to_string(),
            image: DEFAULT_IMAGE.to_string(),
            raw_kernel_elf: DEFAULT_RAW_KERNEL_ELF.to_string(),
            elf_ttables: DEFAULT_ELF_TTABLES.to_string(),
            elf_ttables_symbols: DEFAULT_ELF_TTABLES_SYMBOLS.to_string(),
            bsp: DEFAULT_BSP.to_string(),
            kernel_symbols_manifest: DEFAULT_KERNEL_SYMBOLS_MANIFEST.to_string(),
            final_binary: DEFAULT_FINAL_BINARY.to_string(),
        };
    }
}
