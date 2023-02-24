use crate::{
    commands::build::util::{generate_build_env, BuildState},
    config::BuildConfig,
    docker::{run_in_container_quick, QuickContainerOpts},
    logger::Logger,
};

pub const BUILD_LOGGER: Logger = Logger::new("BUILD => SYMBOLS VIRTUAL ADDRESS");

pub async fn find_virtual_address(config: &BuildConfig, state: &BuildState) -> String {
    BUILD_LOGGER.info(format!("Finding virtual address of symbols section..."));

    let env = generate_build_env(&config);

    let command = vec![
        "ruby".to_string(),
        "tools/kernel_symbols_tool/main.rb".to_string(),
        "--get_symbols_section_virt_addr".to_string(),
        "/work/".to_string() + config.elf_ttables_symbols.clone().as_str(),
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

    return logs;
}
