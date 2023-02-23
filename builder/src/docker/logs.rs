use bollard::{container::LogOutput, errors::Error};
use futures::{Stream, StreamExt};

pub async fn get_logs(log_stream: impl Stream<Item = Result<LogOutput, Error>>) -> String {
    let mut it = Box::pin(log_stream.fuse());
    let mut logs = String::new();

    loop {
        let res = it.next().await;

        if res.is_some() {
            let log_result = res.unwrap();

            if log_result.is_ok() {
                let log = log_result.unwrap();

                if let LogOutput::StdOut { message } = log {
                    logs.push_str(&String::from_utf8(message.to_vec()).unwrap());
                    logs.push('\n');
                } else if let LogOutput::StdErr { message } = log {
                    logs.push_str(&String::from_utf8(message.to_vec()).unwrap());
                    logs.push('\n');
                }
            }
        } else {
            break;
        }
    }

    return logs.trim().to_string();
}
