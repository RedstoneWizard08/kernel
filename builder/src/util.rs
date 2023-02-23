use std::env::current_dir;

pub fn get_cwd() -> String {
    return current_dir()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string();
}
