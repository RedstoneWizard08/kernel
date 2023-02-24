use crate::{
    commands::build::util::{generate_build_env, BuildState},
    config::BuildConfig,
    docker::{run_in_container_quick, QuickContainerOpts},
    logger::Logger,
};

pub const BUILD_LOGGER: Logger = Logger::new("BUILD => FINALIZE BINARY");

pub async fn finalize_build(config: &BuildConfig, state: &BuildState) {
    BUILD_LOGGER.info(format!("Finalizing binary..."));

    let env = generate_build_env(&config);

    let command = vec![
        "~/.cargo/bin/rust-objcopy".to_string(),
        "--strip-all".to_string(),
        "-O".to_string(),
        "binary".to_string(),
        "/work/".to_string() + config.elf_ttables_symbols.clone().as_str(),
        "/work/".to_string() + config.final_binary.clone().as_str(),
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
        BUILD_LOGGER.info(format!("Finalize binary build logs: {}", logs));
    }
}
