use crate::{
    commands::build::util::{generate_build_env, BuildState},
    config::BuildConfig,
    docker::{run_in_container_quick, QuickContainerOpts},
    logger::Logger,
};

pub const BUILD_LOGGER: Logger = Logger::new("BUILD => STRIP SYMBOLS");

pub async fn strip_symbols(config: &BuildConfig, state: &BuildState) {
    BUILD_LOGGER.info(format!("Stripping symbols..."));

    let env = generate_build_env(&config);

    let command = vec![
        "~/.cargo/bin/rust-objcopy".to_string(),
        "--strip-all".to_string(),
        "-O".to_string(),
        "binary".to_string(),
        "/work/".to_string() + config.kernel_symbols_elf.clone().as_str(),
        "/work/".to_string() + config.kernel_symbols_elf.clone().as_str() + "_stripped",
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
        BUILD_LOGGER.info(format!("Strip symbols build logs: {}", logs));
    }
}
