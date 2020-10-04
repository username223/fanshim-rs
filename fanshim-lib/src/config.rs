use crate::{MilliCelsius, MILLI_CELSIUS_IN_CELSIUS};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;

#[derive(serde::Deserialize, Debug)]
#[serde(default)]
pub struct General {
    pub milliseconds_delay_between_readings: u64,
    pub number_of_readings_before_action: i32,
    pub output_debug_messages: bool,
}

impl Default for General {
    fn default() -> Self {
        Self {
            milliseconds_delay_between_readings: 500,
            number_of_readings_before_action: 3,
            output_debug_messages: false,
        }
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(default)]
pub struct Fan {
    pub enable_fan_at_degrees_celsius: i32,
    pub disable_fan_at_degrees_celsius: i32,
}

impl Default for Fan {
    fn default() -> Self {
        Self {
            enable_fan_at_degrees_celsius: 65,
            disable_fan_at_degrees_celsius: 55,
        }
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(default)]
pub struct Led {
    pub led_brightness: f32,
    pub fan_status: LedFanStatus,
}

impl Default for Led {
    fn default() -> Self {
        Self {
            led_brightness: 0.1,
            fan_status: LedFanStatus::default(),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(default)]
pub struct LedFanStatus {
    pub led_on_color: (u8, u8, u8),
    pub led_off_color: (u8, u8, u8),
}

impl Default for LedFanStatus {
    fn default() -> Self {
        Self {
            led_on_color: (255, 0, 0),
            led_off_color: (0, 255, 0),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(default)]
pub struct LedTemperatureStatus {
    led_low_temperature_celsius: i32,
    led_high_temperature_celsius: i32,
}

impl Default for LedTemperatureStatus {
    fn default() -> Self {
        Self {
            led_low_temperature_celsius: 32,
            led_high_temperature_celsius: 80,
        }
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(default)]
pub struct FanshimInterimConfig {
    pub fan: Fan,
    pub general: General,
    pub led: Led,
}

impl Default for FanshimInterimConfig {
    fn default() -> Self {
        Self {
            fan: Default::default(),
            general: Default::default(),
            led: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct FanshimConfig {
    pub delay_between_readings: Duration,
    pub number_of_readings_before_action: i32,
    pub output_debug_messages: bool,
    pub enable_fan_at_temperature: MilliCelsius,
    pub disable_fan_at_temperature: MilliCelsius,
    pub led_brightness: f32,
    pub led_on_color: (u8, u8, u8),
    pub led_off_color: (u8, u8, u8),
}

impl Default for FanshimConfig {
    fn default() -> Self {
        FanshimConfig::from(FanshimInterimConfig::default())
    }
}

impl From<FanshimInterimConfig> for FanshimConfig {
    fn from(f: FanshimInterimConfig) -> Self {
        Self {
            delay_between_readings: Duration::from_millis(
                f.general.milliseconds_delay_between_readings,
            ),
            number_of_readings_before_action: 3,
            output_debug_messages: f.general.output_debug_messages,
            enable_fan_at_temperature: MilliCelsius(
                f.fan.enable_fan_at_degrees_celsius * MILLI_CELSIUS_IN_CELSIUS,
            ),
            disable_fan_at_temperature: MilliCelsius(
                f.fan.disable_fan_at_degrees_celsius * MILLI_CELSIUS_IN_CELSIUS,
            ),
            led_brightness: f.led.led_brightness,
            led_on_color: f.led.fan_status.led_on_color,
            led_off_color: f.led.fan_status.led_off_color,
        }
    }
}

pub fn read_config_file(file_location: &Path) -> crate::Result<FanshimConfig> {
    let mut f = File::open(file_location)?;
    let mut s = String::with_capacity(f.metadata()?.len() as usize);
    File::read_to_string(&mut f, &mut s)?;

    let config: FanshimInterimConfig = toml::from_str(&s)?;

    Ok(FanshimConfig::from(config))
}
