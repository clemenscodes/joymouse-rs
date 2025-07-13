use std::{
  path::PathBuf,
  sync::{LazyLock, OnceLock},
  time::Duration,
};

use config::{Config, File};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};

pub const MAX_STICK_TILT: f64 = 32767.0;
pub const MIN_STICK_TILT: f64 = -32768.0;
pub const LEFT_STICK_SENSITIVITY: f64 = 10000.0;

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
  #[serde(deserialize_with = "from_millis", serialize_with = "to_millis")]
  tickrate: Duration,
  #[serde(deserialize_with = "from_millis", serialize_with = "to_millis")]
  mouse_idle_timeout: Duration,
  max_tilt_range: f64,
  min_tilt_range: f64,
  sensitivity: f64,
  blend: f64,
  diagonal_boost: f64,
  angle_delta_limit: f64,
  speed_stabilize_threshold: f64,
  min_speed_clamp: f64,
  max_speed_clamp: f64,
  motion_threshold_micro_macro: f64,
  motion_threshold_macro_flick: f64,
  motion_threshold_macro_micro: f64,
  motion_threshold_micro_macro_recover: f64,
}

impl Default for ControllerSettings {
  fn default() -> Self {
    let tickrate = Duration::from_millis(16);
    let mouse_idle_timeout = tickrate * 4;
    let minimum_tilt = 0.4;
    let maximum_tilt = 1.0;
    let max_tilt_range = MAX_STICK_TILT * maximum_tilt;
    let min_tilt_range = MAX_STICK_TILT * minimum_tilt;
    Self {
      tickrate,
      mouse_idle_timeout,
      max_tilt_range: max_tilt_range.round(),
      min_tilt_range: min_tilt_range.round(),
      sensitivity: 7.0,
      blend: 0.2,
      diagonal_boost: 1.41,
      angle_delta_limit: 0.5,
      speed_stabilize_threshold: 200.0,
      min_speed_clamp: 1.0,
      max_speed_clamp: 500.0,
      motion_threshold_micro_macro: 0.025,
      motion_threshold_macro_flick: 0.5,
      motion_threshold_macro_micro: 0.03,
      motion_threshold_micro_macro_recover: 0.01,
    }
  }
}

impl ControllerSettings {
  pub const fn tickrate(&self) -> Duration {
    self.tickrate
  }

  pub const fn sensitivity(&self) -> f64 {
    self.sensitivity
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

  pub fn diagonal_boost(&self) -> f64 {
    self.diagonal_boost
  }

  pub fn angle_delta_limit(&self) -> f64 {
    self.angle_delta_limit
  }

  pub fn speed_stabilize_threshold(&self) -> f64 {
    self.speed_stabilize_threshold
  }

  pub fn min_speed_clamp(&self) -> f64 {
    self.min_speed_clamp
  }

  pub fn max_speed_clamp(&self) -> f64 {
    self.max_speed_clamp
  }

  pub fn motion_threshold_micro_macro(&self) -> f64 {
    self.motion_threshold_micro_macro
  }

  pub fn motion_threshold_macro_flick(&self) -> f64 {
    self.motion_threshold_macro_flick
  }

  pub fn motion_threshold_macro_micro(&self) -> f64 {
    self.motion_threshold_macro_micro
  }

  pub fn motion_threshold_micro_macro_recover(&self) -> f64 {
    self.motion_threshold_micro_macro_recover
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
