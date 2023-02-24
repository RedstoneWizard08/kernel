use crate::{
    commands::build::util::{generate_build_env, BuildState},
    config::BuildConfig,
    docker::{run_in_container_quick, QuickContainerOpts},
    logger::Logger,
};

pub const BUILD_LOGGER: Logger = Logger::new("BUILD => TRANSLATION TABLES");

pub async fn build_ttables(config: &BuildConfig, state: &BuildState) {
    BUILD_LOGGER.info(format!("Building translation tables..."));

    let env = generate_build_env(&config);

    let command = vec![
        "ruby".to_string(),
        "tools/translation_table_tool/main.rb".to_string(),
        config.bsp.clone(),
        "/work/".to_string() + config.elf_ttables.clone().as_str(),
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
        BUILD_LOGGER.info(format!("Translation tables build logs: {}", logs));
    }
}
