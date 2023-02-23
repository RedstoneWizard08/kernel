use std::process::exit;

use bollard::{
    container::{Config, LogsOptions},
    service::{HostConfig, Mount, MountTypeEnum},
    Docker,
};

use crate::{logger::Logger, util::get_cwd};

use super::{get_logs, wait_stream};

#[derive(Debug, Clone)]
pub struct ContainerMount {
    pub to: String,
    pub from: String,
}

#[derive(Debug, Clone)]
pub struct ContainerOptions {
    pub cmd: Vec<String>,
    pub env: Vec<String>,
    pub image: String,

    pub mounts: Vec<ContainerMount>,
    pub work_dir: String,
}

#[derive(Debug, Clone)]
pub struct QuickContainerOpts {
    pub cmd: Vec<String>,
    pub env: Vec<String>,
    pub image: String,
}

pub async fn run_in_container_quick(
    opts: QuickContainerOpts,
    docker: Docker,
    logger: Logger,
    verbose: bool,
) -> String {
    let command = vec!["bash".to_string(), "-c".to_string(), opts.cmd.join(" ")];

    let mounts_vec = vec![ContainerMount {
        to: "/work".to_string(),
        from: get_cwd(),
    }];

    let real_opts = ContainerOptions {
        cmd: command,
        env: opts.env,
        image: opts.image,

        mounts: mounts_vec,
        work_dir: "/work".to_string(),
    };

    return run_in_container(real_opts, docker, logger, verbose).await;
}

pub async fn run_in_container(
    opts: ContainerOptions,
    docker: Docker,
    logger: Logger,
    verbose: bool,
) -> String {
    let mut mounts: Vec<Mount> = Vec::new();

    for mount in opts.mounts {
        mounts.push(Mount {
            source: Some(mount.from),
            target: Some(mount.to),
            typ: Some(MountTypeEnum::BIND),
            ..Default::default()
        });
    }

    let host_config = HostConfig {
        mounts: Some(mounts),

        ..Default::default()
    };

    let container_config = Config {
        image: Some(opts.image),

        cmd: Some(opts.cmd.clone()),
        env: Some(opts.env.clone()),

        attach_stderr: Some(verbose),
        attach_stdout: Some(verbose),

        host_config: Some(host_config),
        working_dir: Some(opts.work_dir),

        ..Default::default()
    };

    if verbose {
        let mut command_str = String::new();

        for part in opts.cmd {
            command_str.push_str(part.as_str());
            command_str.push_str(" ");
        }

        let mut env_str = String::new();

        for part in opts.env {
            env_str.push_str(part.as_str());
            env_str.push_str(" ");
        }

        logger.debug("Creating container...".to_string());
        logger.debug(format!("Container start command: {}", command_str.trim()));
        logger.debug(format!("Container env variables: {}", env_str.trim()));
    }

    let container_result = docker
        .create_container::<String, String>(None, container_config)
        .await;

    if container_result.is_err() {
        logger.error(format!(
            "Could not create the container! Error log: {}",
            container_result.unwrap_err()
        ));

        exit(1);
    }

    let container_response = container_result.unwrap();
    let container_id = container_response.id.as_str();

    if verbose {
        logger.debug("Starting the container...".to_string());
    }

    let container_start = docker.start_container::<String>(container_id, None).await;

    if container_start.is_err() {
        logger.error(format!(
            "Could not start the container! Error log: {}",
            container_start.unwrap_err()
        ));

        let container_delete = docker.remove_container(container_id, None).await;

        if container_delete.is_err() {
            logger.error(format!(
                "Could not delete the container! Error log: {}",
                container_delete.unwrap_err()
            ));
        }

        exit(1);
    }

    if verbose {
        logger.debug("Waiting for the container...".to_string());
    }

    let mut container_stream = docker.wait_container::<String>(container_id, None);

    wait_stream(&mut container_stream).await;

    if verbose {
        logger.debug("Deleting the container...".to_string());
    }

    let logs = docker.logs::<String>(
        container_id,
        Some(LogsOptions {
            stdout: true,
            stderr: true,

            ..Default::default()
        }),
    );

    let real_logs = get_logs(logs).await;

    let container_delete = docker.remove_container(container_id, None).await;

    if container_delete.is_err() {
        logger.error(format!(
            "Could not delete the container! Error log: {}",
            container_delete.unwrap_err()
        ));

        exit(1);
    }

    return real_logs;
}
