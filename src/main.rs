mod osu_collector;

#[tokio::main]
async fn main() {
    let client = osu_collector::OsuCollector::default();

    client.download(1680649).await;
    log::info!("{:?}", client.get_collection(6600).await);
}
