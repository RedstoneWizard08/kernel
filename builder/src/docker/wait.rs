use bollard::{errors::Error, service::ContainerWaitResponse};
use futures::{Stream, StreamExt};

pub async fn wait_stream(stream: impl Stream<Item = Result<ContainerWaitResponse, Error>>) {
    let mut it = Box::pin(stream.fuse());

    loop {
        let res = it.next().await;

        if res.is_none() {
            break;
        }
    }
}
