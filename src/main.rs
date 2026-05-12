#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
// #![deny(clippy::large_stack_frames)]

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    log::info!("[PANIC] {}", panic_info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

extern crate alloc;

// -------------

mod app;
mod hardware;
mod models;
mod services;
mod ui;

use embassy_executor::Spawner;
use embassy_net::StackResources;
use esp_hal::rng::Rng;
use esp_hal::rtc_cntl::Rtc;
use esp_radio::wifi::WifiController;
use static_cell::StaticCell;

use crate::app::App;
use crate::hardware::backlight::ledc::Backlight;
use crate::hardware::board::Board;
use crate::hardware::display::display_controller::DisplayController;
use crate::hardware::display::spi_display::SpiDisplayBuilder;
use crate::hardware::radio::wifi::wifi_task;
use crate::models::clock::Clock;
use crate::models::configs::WifiConfig;
use crate::services::embassy_net::embassy_net_task;
use crate::services::ntp::ntp_task;

static STACK_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
static WIFI_CONTROLLER: StaticCell<WifiController<'static>> = StaticCell::new();
static RTC: StaticCell<Rtc<'static>> = StaticCell::new();

const DISPLAY_BUFFER_SIZE: usize = 2048;
static mut DISPLAY_BUFFER: [u8; DISPLAY_BUFFER_SIZE] = [0u8; DISPLAY_BUFFER_SIZE];

#[allow(clippy::large_stack_frames)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // Pin assignments and peripheral configuration can be changed in src/hardware/board.rs
    let board = Board::init();
    let rtc: &'static Rtc<'static> = RTC.init(Rtc::new(board.app_peripherals.lpwr));

    let clock = Clock::default(rtc);
    let rng = Rng::new();

    let wifi_config = WifiConfig {
        ssid: heapless::String::try_from("YOUR_SSID").unwrap(),
        password: heapless::String::try_from("YOUR_SSID_PASSWORD").unwrap(),
    };

    let (wifi_controller_inner, wifi_interfaces) =
        esp_radio::wifi::new(board.app_peripherals.wifi, Default::default()).unwrap();
    let wifi_controller: &'static mut WifiController<'static> =
        WIFI_CONTROLLER.init(wifi_controller_inner);

    let stack_resources = STACK_RESOURCES.uninit().write(StackResources::<3>::new());
    let embassy_net_config = embassy_net::Config::dhcpv4(Default::default());
    let embassy_net_seed = (rng.random() as u64) << 32 | rng.random() as u64;
    let (network_stack, network_runner) = embassy_net::new(
        wifi_interfaces.station,
        embassy_net_config,
        stack_resources,
        embassy_net_seed,
    );

    spawner.spawn(wifi_task(wifi_controller, wifi_config).unwrap());
    spawner.spawn(embassy_net_task(network_runner).unwrap());
    spawner.spawn(ntp_task(network_stack, rtc).unwrap());

    let display_buffer: &'static mut [u8; DISPLAY_BUFFER_SIZE] =
        unsafe { &mut *core::ptr::addr_of_mut!(DISPLAY_BUFFER) };

    // -------------
    let mut backlight = Backlight::new(board.app_peripherals.ledc, board.backlight_config);
    let backlight_controller = backlight.get_controller();

    let display = SpiDisplayBuilder::build(
        board.app_peripherals.spi,
        board.display_config,
        display_buffer,
    );
    let display_controller = DisplayController::new(display, Some(backlight_controller));

    let mut app = App::new(display_controller, &clock);
    app.run(spawner).await;
}
