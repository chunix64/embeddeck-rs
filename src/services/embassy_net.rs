#[embassy_executor::task]
pub async fn embassy_net_task(
    mut runner: embassy_net::Runner<'static, esp_radio::wifi::Interface<'static>>,
) {
    runner.run().await
}
