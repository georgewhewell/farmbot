use crate::config;
use crate::mqtt::{MQTTBinarySensorConfig, MQTTDevice};
use anyhow::anyhow;
use anyhow::Result;
use i2cdev::core::*;
use i2cdev::linux::LinuxI2CDevice;
use linux_embedded_hal::I2cdev;
use log::{debug, info};
use serde_json::json;
use std::thread;
use std::time;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct TankLevelSensor {
    have_water: bool,
    last_read: Instant,
}

const ADDRESS: u16 = 0x4;
const BUS: &'static str = "/dev/i2c-0";

const PIN: u8 = 0;
const MODE_COMMAND: u8 = 5;
const READ_COMMAND: u8 = 3;

fn read() -> Result<bool> {
    debug!("Attempt to read tank low");
    let mut dev = LinuxI2CDevice::new(&BUS, ADDRESS)?;

    debug!("Setting PIN to INPUT");
    dev.smbus_write_i2c_block_data(0, &[MODE_COMMAND, PIN, 0, 0])?;

    std::thread::sleep(time::Duration::from_millis(100));
    debug!("Performing read");

    dev.smbus_write_i2c_block_data(0, &[READ_COMMAND, PIN, 0, 0])?;
    std::thread::sleep(time::Duration::from_millis(100));

    debug!("Reading response.. ");
    dev.smbus_read_byte().unwrap();

    std::thread::sleep(time::Duration::from_millis(100));

    let res = dev.smbus_read_i2c_block_data(0, 4)?;
    let result = res[1] as u32 * 256 + res[2] as u32;

    debug!("Raw: {:?}", res);
    debug!("Result: {:?}", result);

    info!("Level was {}, we have water? {}", result, result > 1000);
    Ok(result > 1000)
}

impl TankLevelSensor {
    pub fn new(config: &config::FarmbotConfig) -> Result<Self> {
        info!("Creating tank level sensor");
        let raw = read()?;
        return Ok(TankLevelSensor {
            have_water: false,
            last_read: Instant::now(),
        });
    }

    pub fn should_read(&self) -> bool {
        self.last_read + Duration::from_millis(30000) < Instant::now()
    }

    pub fn get(&self) -> bool {
        self.have_water
    }

    pub fn mqtt_config(&self) -> MQTTBinarySensorConfig {
        MQTTBinarySensorConfig {
            name: "Water Tank".to_string(),
            unique_id: "farmbot-watertank".to_string(),
            device_class: "moisture".to_string(),
            state_topic: "homeassistant/sensor/farmbot/watertank/state".to_string(),
            device: MQTTDevice {
                identifiers: vec!["farmbot".to_string()],
                connections: vec![],
                manufacturer: "Whewell Comms".to_string(),
                name: "Farmbot".to_string(),
                model: "Farmbot 0.1".to_string(),
            },
        }
    }

    pub fn mqtt_state(have_water: bool) -> serde_json::Value {
        json!(if have_water { "OFF" } else { "ON" })
    }

    pub fn read(&mut self) -> Result<bool> {
        info!("Attempt to read from sensor");
        let have_water = read()?;
        self.last_read = Instant::now();
        self.have_water = have_water;
        Ok(have_water)
    }
}
