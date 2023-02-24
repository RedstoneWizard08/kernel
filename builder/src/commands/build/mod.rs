pub mod util;
pub mod patch;
pub mod strip;
pub mod kernel;
pub mod symbols;
pub mod finalize;

use self::{
    kernel::build_kernel,
    patch::patch_elf,
    strip::strip_binary,
    symbols::{
        build_demangled::build_demangled_symbools,
        copiers::{copy_elf_ttables, copy_new_elf_ttables},
        debug_symbols::build_debug_symbols,
        demangle::demangle,
        strip::strip_symbols,
        translation_tables::build_ttables,
        virtual_address::find_virtual_address,
    },
    util::BuildState, finalize::finalize_build,
};

use super::clean::run_clean;
use crate::{config::BuildConfig, docker::connect_default, logger::Logger};
use std::process::exit;

pub const BUILD_LOGGER: Logger = Logger::new("BUILD");

pub async fn run_build(clean: bool, verbose: bool) {
    if clean {
        run_clean(verbose);
    }

    BUILD_LOGGER.info(format!("Connecting to docker..."));

    let config = BuildConfig::default();
    let docker_result = connect_default();

    if docker_result.is_err() {
        BUILD_LOGGER.error(format!(
            "Could not connect to docker! Error log: {}",
            docker_result.unwrap_err()
        ));

        exit(1);
    }

    let docker = docker_result.unwrap();
    let version = docker.version().await.unwrap();

    BUILD_LOGGER.info(format!("Using docker {}.", version.version.unwrap()));

    let state = BuildState {
        docker,
        clean,
        verbose,
    };

    build_kernel(&config, &state).await;
    copy_elf_ttables(&config);
    build_ttables(&config, &state).await;
    copy_new_elf_ttables(&config);
    build_debug_symbols(&config, &state).await;
    demangle(&config);

    let addr = find_virtual_address(&config, &state).await;

    build_demangled_symbools(&config, &state, addr).await;

    strip_symbols(&config, &state).await;
    strip_binary(&config, &state).await;
    patch_elf(&config, &state).await;
    finalize_build(&config, &state).await;
}
