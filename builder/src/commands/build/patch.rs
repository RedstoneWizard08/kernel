use crate::{
    commands::build::util::{generate_build_env, BuildState},
    config::BuildConfig,
    docker::{run_in_container_quick, QuickContainerOpts},
    logger::Logger,
};

pub const BUILD_LOGGER: Logger = Logger::new("BUILD => PATCH ELF");

pub async fn patch_elf(config: &BuildConfig, state: &BuildState) {
    BUILD_LOGGER.info(format!("Patching final ELF..."));

    let env = generate_build_env(&config);

    let command = vec![
        "ruby".to_string(),
        "tools/kernel_symbols_tool/main.rb".to_string(),
        "--patch_data".to_string(),
        config.elf_ttables_symbols.clone(),
        "/work/".to_string() + config.raw_kernel_elf.clone().as_str() + "_symbols_stripped",
    ];

    let logs = run_in_container_quick(
        QuickContainerOpts {
            cmd: command,
            env,
            image: config.image.clone(),
        },
        state.docker.clone(),
        BUILD_LOGGER,
        state.verbose,
    )
    .await;

    if state.verbose {
        BUILD_LOGGER.info(format!("Patch ELF build logs: {}", logs));
    }
}
