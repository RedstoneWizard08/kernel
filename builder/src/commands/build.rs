use crate::{docker::connect_default, logger::Logger};
use bollard::{
    container::Config,
    errors::Error,
    service::{ContainerWaitResponse, HostConfig, Mount, MountTypeEnum},
};
use futures_core::Stream;
use std::env::current_dir;
use tokio_stream::StreamExt;

pub const BUILD_LOGGER: Logger = Logger::new("BUILD");

pub const DEFAULT_CPU: &'static str = "cortex-a72";
pub const DEFAULT_LD_SCRIPT_PATH: &'static str = "/work/kernel/src/bsp/aarch64";
pub const DEFAULT_KERNEL_LINKER_SCRIPT: &'static str = "kernel.ld";
pub const DEFAULT_TARGET: &'static str = "aarch64-unknown-none-softfloat";
pub const DEFAULT_KERNEL_SYMBOLS_ELF: &'static str =
    "/work/target/aarch64-unknown-none-softfloat/release/kernel_symbols";
pub const DEFAULT_KERNEL_MANIFEST: &'static str = "/work/kernel/Cargo.toml";
pub const DEFAULT_IMAGE: &'static str = "docker.io/redstonewizard/kernel-builder:latest";

pub fn get_cwd() -> String {
    return current_dir()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string();
}

pub fn generate_rust_flags() -> String {
    let mut flags: Vec<String> = Vec::new();

    flags.push("-C".to_string());
    flags.push("target-cpu=".to_string() + DEFAULT_CPU);
    flags.push("-C".to_string());
    flags.push("force-frame-pointers".to_string());
    flags.push("-C".to_string());
    flags.push("link-arg=--library-path=".to_string() + DEFAULT_LD_SCRIPT_PATH);
    flags.push("-C".to_string());
    flags.push("link-arg=--script=".to_string() + DEFAULT_KERNEL_LINKER_SCRIPT);

    return flags.join(" ");
}

pub struct BuildConfig {
    rust_flags: String,
    ld_script_path: String,
    target: String,
    kernel_symbols_elf: String,
    kernel_manifest: String,
    image: String,
}

impl Default for BuildConfig {
    fn default() -> Self {
        return Self {
            rust_flags: generate_rust_flags(),
            ld_script_path: DEFAULT_LD_SCRIPT_PATH.to_string(),
            target: DEFAULT_TARGET.to_string(),
            kernel_symbols_elf: DEFAULT_KERNEL_SYMBOLS_ELF.to_string(),
            kernel_manifest: DEFAULT_KERNEL_MANIFEST.to_string(),
            image: DEFAULT_IMAGE.to_string(),
        };
    }
}

pub async fn wait_stream(stream: impl Stream<Item = Result<ContainerWaitResponse, Error>>) {
    // let pinned = Box::pin(stream);
    let mut it = Box::pin(stream.fuse());

    loop {
        let res = it.next().await;

        if res.is_none() {
            break;
        } else {
            let next = res.unwrap();

            if next.is_ok() {
                let val = next.unwrap();

                BUILD_LOGGER.info(format!("Wait response: {:?}", val));
            }
        }
    }
}

pub async fn run_build(clean: bool, verbose: bool) {
    BUILD_LOGGER.info(format!("Connecting to docker..."));

    let config = BuildConfig::default();
    let docker_result = connect_default();

    if docker_result.is_err() {
        BUILD_LOGGER.error(format!(
            "Could not connect to docker! Error log: {}",
            docker_result.unwrap_err()
        ));

        return;
    }

    let docker = docker_result.unwrap();
    let version = docker.version().await.unwrap();

    BUILD_LOGGER.info(format!("Using docker {}.", version.version.unwrap()));

    let env = vec![
        "RUSTFLAGS=".to_string() + config.rust_flags.as_str(),
        "LD_SCRIPT_PATH=".to_string() + config.ld_script_path.as_str(),
        "KERNEL_SYMBOLS_DEMANGLED_RS=".to_string()
            + config.kernel_symbols_elf.as_str()
            + "_demangled.rs",
    ];

    let command_split = vec![
        "cargo".to_string(),
        "rustc".to_string(),
        "--target".to_string(),
        config.target,
        "--features".to_string(),
        "bsp_rpi4".to_string(),
        "--release".to_string(),
        "-Z".to_string(),
        "build-std=core,alloc".to_string(),
        "--manifest-path".to_string(),
        config.kernel_manifest,
    ];

    let command = vec![
        "bash".to_string(),
        "-c".to_string(),
        command_split.join(" "),
    ];

    let mut mounts: Vec<Mount> = Vec::new();

    mounts.push(Mount {
        source: Some(get_cwd()),
        target: Some("/work".to_string()),
        typ: Some(MountTypeEnum::BIND),
        ..Default::default()
    });

    let host_config = HostConfig {
        mounts: Some(mounts),

        ..Default::default()
    };

    let container_config = Config {
        image: Some(config.image),

        cmd: Some(command.clone()),
        env: Some(env.clone()),

        attach_stderr: Some(verbose),
        attach_stdout: Some(verbose),

        host_config: Some(host_config),
        working_dir: Some("/work".to_string()),

        ..Default::default()
    };

    if verbose {
        let mut command_str = String::new();

        for part in command {
            command_str.push_str(part.as_str());
            command_str.push_str(" ");
        }

        let mut env_str = String::new();

        for part in env {
            env_str.push_str(part.as_str());
            env_str.push_str(" ");
        }

        BUILD_LOGGER.debug("Creating container...".to_string());
        BUILD_LOGGER.debug(format!("Container start command: {}", command_str.trim()));
        BUILD_LOGGER.debug(format!("Container env variables: {}", env_str.trim()));
    }

    let container_result = docker
        .create_container::<String, String>(None, container_config)
        .await;

    if container_result.is_err() {
        BUILD_LOGGER.error(format!(
            "Could not create the container! Error log: {}",
            container_result.unwrap_err()
        ));

        return;
    }

    let container_response = container_result.unwrap();
    let container_id = container_response.id.as_str();

    if verbose {
        BUILD_LOGGER.debug("Starting the container...".to_string());
    }

    let container_start = docker.start_container::<String>(container_id, None).await;

    if container_start.is_err() {
        BUILD_LOGGER.error(format!(
            "Could not start the container! Error log: {}",
            container_start.unwrap_err()
        ));

        let container_delete = docker.remove_container(container_id, None).await;

        if container_delete.is_err() {
            BUILD_LOGGER.error(format!(
                "Could not delete the container! Error log: {}",
                container_delete.unwrap_err()
            ));
        }

        return;
    }

    if verbose {
        BUILD_LOGGER.debug("Waiting for the container...".to_string());
    }

    let mut container_stream = docker.wait_container::<String>(container_id, None);

    wait_stream(&mut container_stream).await;

    if verbose {
        BUILD_LOGGER.debug("Deleting the container...".to_string());
    }

    let container_delete = docker.remove_container(container_id, None).await;

    if container_delete.is_err() {
        BUILD_LOGGER.error(format!(
            "Could not delete the container! Error log: {}",
            container_delete.unwrap_err()
        ));

        return;
    }
}
