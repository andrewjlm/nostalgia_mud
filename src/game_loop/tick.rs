use tokio::time::{sleep, Duration};

pub async fn tick() {
    // TODO: Figure out proper tick length
    log::debug!("Tick!");
    sleep(Duration::from_millis(1000)).await
}
