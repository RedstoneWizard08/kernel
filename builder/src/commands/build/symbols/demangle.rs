use crate::{config::BuildConfig, demangler::do_demangle, logger::Logger};

pub const BUILD_LOGGER: Logger = Logger::new("BUILD => DEMANGLER");

pub fn demangle(config: &BuildConfig) {
    BUILD_LOGGER.info(format!("Demangling debug symbols..."));

    let in_file = config.elf_ttables.clone() + "_symbols.rs";
    let out_file = config.elf_ttables.clone() + "_symbols_demangled.rs";

    do_demangle(in_file, out_file);
}
