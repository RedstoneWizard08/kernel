use super::util::{generate_build_env, BuildState};
use crate::{
    config::BuildConfig,
    docker::{run_in_container_quick, QuickContainerOpts},
    logger::Logger,
};

pub const BUILD_LOGGER: Logger = Logger::new("BUILD => KERNEL");

pub async fn build_kernel(config: &BuildConfig, state: &BuildState) {
    let env = generate_build_env(&config);

    let command = vec![
        "~/.cargo/bin/cargo".to_string(),
        "rustc".to_string(),
        "--target".to_string(),
        config.target.clone(),
        "--features".to_string(),
        "bsp_rpi4".to_string(),
        "--release".to_string(),
        "-Z".to_string(),
        "build-std=core,alloc".to_string(),
        "--manifest-path".to_string(),
        "/work/".to_string() + config.kernel_manifest.clone().as_str(),
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
        BUILD_LOGGER.info(format!("Kernel build logs: {}", logs));
    }
}
