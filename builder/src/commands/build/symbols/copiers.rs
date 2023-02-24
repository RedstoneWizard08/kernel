use crate::{config::BuildConfig, logger::Logger};
use std::{fs::copy, process::exit};

pub const BUILD_LOGGER: Logger = Logger::new("BUILD => COPIERS");

pub fn copy_elf_ttables(config: &BuildConfig) {
    BUILD_LOGGER.info(format!("Copying ELF translation tables..."));

    let res = copy(config.raw_kernel_elf.clone(), config.elf_ttables.clone());

    if !res.is_ok() {
        BUILD_LOGGER.error(format!(
            "Could not copy ELF translation tables! Error log: {}",
            res.unwrap_err()
        ));

        exit(1);
    }
}

pub fn copy_new_elf_ttables(config: &BuildConfig) {
    BUILD_LOGGER.info(format!("Copying new ELF translation tables..."));

    let res = copy(
        config.elf_ttables.clone(),
        config.elf_ttables_symbols.clone(),
    );

    if !res.is_ok() {
        BUILD_LOGGER.error(format!(
            "Could not copy new ELF translation tables! Error log: {}",
            res.unwrap_err()
        ));

        exit(1);
    }
}
