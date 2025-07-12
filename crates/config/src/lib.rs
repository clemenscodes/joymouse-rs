use std::{
  path::PathBuf,
  sync::{LazyLock, OnceLock},
  time::Duration,
};

use config::{Config, File};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};

fn config() -> &'static Config {
  static CONFIG: OnceLock<Config> = OnceLock::new();
  CONFIG.get_or_init(|| {
    let mut builder = Config::builder();

    if let Some(base_dirs) = BaseDirs::new() {
      let mut config_path = PathBuf::from(base_dirs.config_dir());
      config_path.push("joymouse/joymouse.toml");

      if !config_path.exists() {
        if let Some(parent) = config_path.parent() {
          std::fs::create_dir_all(parent).unwrap();
        }

        let default_settings = ControllerSettings::default();

        let toml_str = {
          toml::to_string_pretty(&default_settings).expect("failed to serialize default config")
        };

        std::fs::write(&config_path, toml_str).expect("failed to write default config file");
      }

      builder = builder.add_source(File::with_name(config_path.to_str().unwrap()).required(true));
    }

    builder.build().unwrap()
  })
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ControllerSettings {
  name: String,
  vendor: u16,
  product: u16,
  version: u16,
  max_stick_tilt: f64,
  min_stick_tilt: f64,
  deadzone: f64,
  noise_tolerance: f64,

  #[serde(deserialize_with = "from_millis", serialize_with = "to_millis")]
  tickrate: Duration,

  left_stick_sensitivity: f64,
  right_stick_sensitivity: f64,
  blend: f64,

  #[serde(deserialize_with = "from_millis", serialize_with = "to_millis")]
  mouse_idle_timeout: Duration,

  max_tilt_range: f64,
  min_tilt_range: f64,
}

impl ControllerSettings {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub const fn vendor(&self) -> u16 {
    self.vendor
  }

  pub const fn product(&self) -> u16 {
    self.product
  }

  pub const fn version(&self) -> u16 {
    self.version
  }

  pub const fn max_stick_tilt(&self) -> f64 {
    self.max_stick_tilt
  }

  pub const fn min_stick_tilt(&self) -> f64 {
    self.min_stick_tilt
  }

  pub const fn deadzone(&self) -> f64 {
    self.deadzone
  }

  pub const fn noise_tolerance(&self) -> f64 {
    self.noise_tolerance
  }

  pub const fn tickrate(&self) -> Duration {
    self.tickrate
  }

  pub const fn left_stick_sensitivity(&self) -> f64 {
    self.left_stick_sensitivity
  }

  pub const fn right_stick_sensitivity(&self) -> f64 {
    self.right_stick_sensitivity
  }

  pub const fn blend(&self) -> f64 {
    self.blend
  }

  pub const fn mouse_idle_timeout(&self) -> Duration {
    self.mouse_idle_timeout
  }

  pub fn max_tilt_range(&self) -> f64 {
    self.max_tilt_range
  }

  pub fn min_tilt_range(&self) -> f64 {
    self.min_tilt_range
  }
}

impl Default for ControllerSettings {
  fn default() -> Self {
    let name = "JoyMouse".to_string();
    let vendor = 0x1234;
    let product = 0x5678;
    let version = 0x0100;
    let deadzone = 0.0;
    let noise_tolerance = 0.0;
    let tickrate = Duration::from_millis(16);
    let mouse_idle_timeout = tickrate * 4;
    let left_stick_sensitivity = 10000.0;
    let right_stick_sensitivity = 7.0;
    let max_stick_tilt = 32767.0;
    let min_stick_tilt = -32768.0;
    let minimum_tilt = 0.4;
    let maximum_tilt = 1.0;
    let blend = 0.2;
    let max_tilt_range = max_stick_tilt * maximum_tilt;
    let min_tilt_range = max_stick_tilt * minimum_tilt;
    Self {
      name,
      vendor,
      product,
      version,
      max_stick_tilt,
      min_stick_tilt,
      deadzone,
      noise_tolerance,
      tickrate,
      left_stick_sensitivity,
      right_stick_sensitivity,
      blend,
      mouse_idle_timeout,
      max_tilt_range,
      min_tilt_range,
    }
  }
}

fn from_millis<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let ms = u64::deserialize(deserializer)?;
  Ok(Duration::from_millis(ms))
}

fn to_millis<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  serializer.serialize_u64(duration.as_millis() as u64)
}

#[rustfmt::skip]
pub static SETTINGS: LazyLock<ControllerSettings> = LazyLock::new(|| {
    config()
        .clone()
        .try_deserialize()
        .expect("invalid config file")
});
