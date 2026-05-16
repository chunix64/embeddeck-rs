use embassy_time::Delay;
use embeddeck_core::models::{units::Temperature, weather::Weather};
use embedded_hal_async::delay::DelayNs;
use log::warn;
use nanofish::{HttpHeader, ResponseBody, SmallHttpClient, mime_types};
use static_cell::StaticCell;

use crate::{helpers::http::unchunk, models::state::AppState};

// TODO: Implement
const WEATHER_URL: &str =
    "http://api.open-meteo.com/v1/forecast?latitude=1.35&longitude=103.81&current=temperature_2m";

static RESPONSE_BUFFER: StaticCell<[u8; 512]> = StaticCell::new();

#[embassy_executor::task]
pub async fn weather_service(
    network_stack: embassy_net::Stack<'static>,
    app_state: &'static AppState,
) {
    network_stack.wait_config_up().await;

    let interval = 60000; // 1 minute

    let client = SmallHttpClient::new(&network_stack);
    let response_buffer = RESPONSE_BUFFER.init([0u8; 512]);
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
                log::debug!(
                    "[DEBUG] the http request is unclean because of Transfer-Encoding: chunked: {:?}",
                    json
                );
                warn!("[WEATHER] Parse JSON failed: {:?}", error);
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

    Some(Weather {
        geo_location: heapless::String::new(),
        temperature: Temperature::celsius(current["temperature_2m"].as_f64()?),
        apparent_temperature: Temperature::celsius(current["apparent_temperature"].as_f64()?),
        relative_humidity: current["relative_humidity_2m"].as_u64()? as u8,
    })
}
