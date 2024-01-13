use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::spi::SPI1;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use esp_idf_hal::prelude::Peripherals;


use log::info;

fn main() -> () {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();
    /*
     https://github.com/esp-rs/esp-idf-hal/blob/master/examples/blinky.rs
     */
    let f = peripherals.pins.gpio35;
    let w = PinDriver::output(f).unwrap().into_output();
    
    

    // let mut wifi = BlockingWifi::wrap(
    //     EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs)).unwrap(),
    //     sys_loop,
    // ).unwrap();

    // connect_wifi(&mut wifi).unwrap();

    // let ip_info = wifi.wifi().sta_netif().get_ip_info().unwrap();

    // info!("Wifi DHCP info: {:?}", ip_info);

    info!("Shutting down in 5s...");

    std::thread::sleep(core::time::Duration::from_secs(5));

    
}
