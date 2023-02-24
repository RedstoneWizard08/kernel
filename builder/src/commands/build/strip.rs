use crate::{
    commands::build::util::{generate_build_env, BuildState},
    config::BuildConfig,
    docker::{run_in_container_quick, QuickContainerOpts},
    logger::Logger,
};

pub const BUILD_LOGGER: Logger = Logger::new("BUILD => STRIP BINARY");

pub async fn strip_binary(config: &BuildConfig, state: &BuildState) {
    BUILD_LOGGER.info(format!("Stripping binary..."));

    let env = generate_build_env(&config);

    let command = vec![
        "~/.cargo/bin/rust-objcopy".to_string(),
        "--strip-all".to_string(),
        "-O".to_string(),
        "binary".to_string(),
        "/work/".to_string() + config.raw_kernel_elf.clone().as_str() + "_symbols",
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
        BUILD_LOGGER.info(format!("Strip binary build logs: {}", logs));
    }
}
