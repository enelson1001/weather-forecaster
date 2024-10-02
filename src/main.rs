pub mod cities_settings_pane;
pub mod file_store;
pub mod forecast_weather_api;
pub mod gt911;
pub mod home_pane;
pub mod lcd_panel;
pub mod lvgl_misc;
pub mod model;
pub mod navigation_pane;
pub mod startup_pane;
pub mod ui;
pub mod wifi_settings_pane;

use crate::file_store::FileStore;
use crate::gt911::GT911;
use crate::model::{Model, ModelRequest};
use crate::ui::{UiRequest, UserInterface};

use core::time::Duration;
use std::sync::mpsc;

use esp_idf_svc::hal::{
    delay::Ets,
    gpio::{self, PinDriver},
    i2c::{I2cConfig, I2cDriver},
    ledc::{
        config::TimerConfig,
        {LedcDriver, LedcTimerDriver},
    },
    peripherals::Peripherals,
    units::FromValueType,
};

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    timer::EspTaskTimerService,
    wifi::{BlockingWifi, EspWifi},
};

use log::info;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("========== Starting App ==========");

    // Create mpsc channels
    let (tx1, rx1) = mpsc::channel::<UiRequest>(); // tx = model,  rx = ui
    let (tx2, rx2) = mpsc::channel::<ModelRequest>(); // tx = ui     rx = model

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    #[allow(unused)]
    let pins = peripherals.pins;

    // Create file store to read/write text files
    let file_store = FileStore::init(
        peripherals.spi2,
        pins.gpio12.into(),
        pins.gpio11.into(),
        pins.gpio13.into(),
        pins.gpio10.into(),
    )
    .unwrap();

    // Create I2C0 for GT911
    let i2c = peripherals.i2c0;
    let sda = pins.gpio19;
    let scl = pins.gpio20;
    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    // Create LedcDriver for LCD panel backlight
    let mut ledc_driver = LedcDriver::new(
        peripherals.ledc.channel0,
        LedcTimerDriver::new(
            peripherals.ledc.timer0,
            &TimerConfig::new().frequency(25.kHz().into()),
        )
        .unwrap(),
        pins.gpio2,
    )
    .unwrap();

    // Create Wifi service
    let wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs)).unwrap(),
        sys_loop.clone(),
    )
    .unwrap();

    // Create GT911 touchscreen driver
    let gt911 = GT911::new(i2c);
    reset_gt911(pins.gpio38.into());

    // Create the user interface
    UserInterface::new(gt911, tx2, rx1).run();

    // Create a periodic timer to call LVGL tick. LVGL needs a system tick to know elapsed time for animations and other tasks.
    //info!("---------- Creating Lvgl Tick Timer ----------");
    let timer_service_01 = EspTaskTimerService::new().unwrap();
    let lvgl_tick_timer = timer_service_01
        .timer(move || {
            lvgl::tick_inc(Duration::from_millis(10));
        })
        .unwrap();

    // Let it trigger every 10 milliseconds
    lvgl_tick_timer.every(Duration::from_millis(10)).unwrap();

    // Turn On LCD backlight
    ledc_driver.set_duty(ledc_driver.get_max_duty()).unwrap();

    info!("Creating Model");
    let mut model = Model::new(wifi, rx2, tx1, file_store);
    model.run();
}

// Reset the GT911 chip
fn reset_gt911(rst_pin: gpio::AnyOutputPin) {
    let mut rst = PinDriver::output(rst_pin).unwrap();
    rst.set_low().unwrap();
    Ets::delay_us(200);
    rst.set_high().unwrap();
    Ets::delay_ms(5);
}
