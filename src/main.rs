mod boat;
mod server;
mod wifi;

use anyhow::Result;
use log::{debug, info};
use std::{thread::sleep, time::Duration};

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");
const WIFI_AP_MODE: &str = env!("WIFI_AP_MODE");

fn main() -> Result<()> {
    // Initialize ESP-IDF system and logging.
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("connecting to wifi...");
    let wifi = wifi::setup_wifi(WIFI_SSID, WIFI_PASSWORD, WIFI_AP_MODE == "1")?;
    sleep(Duration::from_secs(1));
    let ip = wifi.sta_netif().get_ip_info()?.ip;
    info!("connected to wifi with ip: {ip:?}");

    // Create the boat and its components.
    let boat = boat::Boat {
        motor: boat::motor::setup_motor()?,
        rudder: boat::rudder::setup_rudder()?,
    };

    // Set up http server and keep a reference to it (otherwise it drops out of scope).
    let _server = server::setup_server(boat)?;

    // Keep the main thread alive by sleeping periodically.
    loop {
        debug!("server is still running.");
        sleep(Duration::from_secs(10));
    }
}
