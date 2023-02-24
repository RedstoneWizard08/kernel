use crate::{
    commands::build::util::{generate_demangle_build_env, BuildState},
    config::BuildConfig,
    docker::{run_in_container_quick, QuickContainerOpts},
    logger::Logger,
};

pub const BUILD_LOGGER: Logger = Logger::new("BUILD => DEMANGLED SYMBOLS");

pub async fn build_demangled_symbools(config: &BuildConfig, state: &BuildState, address: String) {
    BUILD_LOGGER.info(format!("Building demangled symbols..."));

    let env = generate_demangle_build_env(&config, address);

    let command = vec![
        "~/.cargo/bin/cargo".to_string(),
        "rustc".to_string(),
        "--target".to_string(),
        config.target.clone(),
        "--release".to_string(),
        "--manifest-path".to_string(),
        "/work/".to_string() + config.kernel_symbols_manifest.clone().as_str(),
    ];

    run_in_container_quick(
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
}
