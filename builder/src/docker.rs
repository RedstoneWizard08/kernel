use std::path::Path;

use bollard::{errors::Error, Docker};
use std::env::var;

pub fn connect(socket: bool) -> Result<Docker, Error> {
    if socket {
        return Docker::connect_with_socket_defaults();
    } else {
        return Docker::connect_with_http_defaults();
    }
}

pub fn connect_default() -> Result<Docker, Error> {
    let socket_path_str = var("DOCKER_SOCKET_PATH").unwrap_or("/var/run/docker.sock".to_string());
    let socket_path = Path::new(&socket_path_str);

    if socket_path.exists() {
        return connect(true);
    } else {
        return connect(false);
    }
}
