use embassy_time::Delay;
use embeddeck_core::models::{units::Temperature, weather::Weather};
use embedded_hal_async::delay::DelayNs;
use log::warn;
use nanofish::{HttpHeader, ResponseBody, SmallHttpClient, mime_types};
use static_cell::StaticCell;

use crate::{helpers::http::unchunk, models::state::AppState};

// TODO: Implement
const WEATHER_URL: &str = "http://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&current=temperature_2m,relative_humidity_2m,apparent_temperature";

static RESPONSE_BUFFER: StaticCell<[u8; 1024]> = StaticCell::new();

#[embassy_executor::task]
pub async fn weather_service(
    network_stack: embassy_net::Stack<'static>,
    app_state: &'static AppState,
) {
    network_stack.wait_config_up().await;

    let interval = 60000; // 1 minute

    let client = SmallHttpClient::new(&network_stack);
    let response_buffer = RESPONSE_BUFFER.init([0u8; 1024]);
    let mut headers = [
        HttpHeader::user_agent("Embeddeck/0.1.0"),
        HttpHeader::content_type(mime_types::JSON),
        HttpHeader::new("Connection", "close"),
    ];

    loop {
        if let Some(weather) = fetch_weather(&client, &mut headers, response_buffer).await {
            let mut guard = app_state.weather.lock().await;
            *guard = Some(weather);
        } else {
            warn!("[WEATHER] Fetch weather API failed, retry!");
        };
        Delay.delay_ms(interval).await;
    }
}

async fn fetch_weather(
    client: &SmallHttpClient<'_>,
    headers: &mut [HttpHeader<'_>],
    response_buffer: &mut [u8],
) -> Option<Weather> {
    let (response, _bytes_read) = match client.get(WEATHER_URL, headers, response_buffer).await {
        Ok(result) => result,
        Err(error) => {
            warn!("[WEATHER] HTTP fetch failed: {:?}", error);
            return None;
        }
    };

    let json_data: serde_json::Value = match &response.body {
        ResponseBody::Text(json) => match serde_json::from_str(unchunk(json)) {
            Ok(json_data) => json_data,
            Err(error) => {
                warn!("[WEATHER] Parse JSON failed: {:?}", error,);
                return None;
            }
        },
        ResponseBody::Binary(binary) => {
            warn!("[WEATHER] Unknown binary response: {:?}", binary);
            return None;
        }
        ResponseBody::Empty => {
            warn!("[WEATHER] Fetch response is empty!");
            return None;
        }
    };

    let current = &json_data["current"];
    let temperature = current["temperature_2m"].as_f64().map(Temperature::celsius);
    let apparent_temperature = current["apparent_temperature"]
        .as_f64()
        .map(Temperature::celsius);
    let relative_humidity = current["relative_humidity_2m"]
        .as_u64()
        .map(|value| value as u8);

    Some(Weather {
        geo_location: None,
        temperature,
        apparent_temperature,
        relative_humidity,
    })
}
