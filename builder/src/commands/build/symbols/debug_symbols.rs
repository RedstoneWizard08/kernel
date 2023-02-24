use crate::{
    commands::build::util::{generate_build_env, BuildState},
    config::BuildConfig,
    docker::{run_in_container_quick, QuickContainerOpts},
    logger::Logger,
};

pub const BUILD_LOGGER: Logger = Logger::new("BUILD => DEBUG SYMBOLS");

pub async fn build_debug_symbols(config: &BuildConfig, state: &BuildState) {
    BUILD_LOGGER.info(format!("Building debug symbols..."));

    let env = generate_build_env(&config);

    let command = vec![
        "ruby".to_string(),
        "tools/kernel_symbols_tool/main.rb".to_string(),
        "--gen_symbols".to_string(),
        "/work/".to_string() + config.elf_ttables_symbols.clone().as_str(),
        "/work/".to_string() + config.elf_ttables.clone().as_str() + "_symbols.rs",
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
        BUILD_LOGGER.info(format!("Debug symbols build logs: {}", logs));
    }
}
