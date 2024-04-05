use tokio::time::{sleep, Duration};

pub async fn tick() {
    // TODO: Figure out proper tick length
    tracing::trace!("Tick!");
    sleep(Duration::from_millis(1000)).await
}
