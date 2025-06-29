use std::time::Instant;

use crate::controller::{
  joystick::{direction::Direction, vector::Vector},
  settings::SETTINGS,
  state::State,
};

#[derive(Debug)]
pub struct JoyStickState {
  x: i32,
  y: i32,
  up: State,
  down: State,
  left: State,
  right: State,
  direction: Option<Direction>,
  last_event: Instant,
}

impl Default for JoyStickState {
  fn default() -> Self {
    Self {
      x: Default::default(),
      y: Default::default(),
      up: Default::default(),
      down: Default::default(),
      left: Default::default(),
      right: Default::default(),
      direction: Default::default(),
      last_event: Instant::now(),
    }
  }
}

impl JoyStickState {
  pub const MAX: i32 = 32767;

  pub fn tilt(&mut self, vector: Vector) -> i32 {
    let sensitivity = SETTINGS.sensitivity();

    println!("Current stick: {:#?}, vector: {:#?}", self, vector);

    self.x += vector.dx() * sensitivity;
    self.y += vector.dy() * sensitivity;

    let magnitude = ((self.x as f64).powi(2) + (self.y as f64).powi(2)).sqrt();
    if magnitude > Self::MAX as f64 {
      let scale = Self::MAX as f64 / magnitude;
      self.x = (self.x as f64 * scale).round() as i32;
      self.y = (self.y as f64 * scale).round() as i32;
    }

    if vector.dx() != 0 {
      self.x
    } else {
      self.y
    }
  }

  pub fn update_direction(&mut self) {
    let up = matches!(self.up, State::Pressed | State::Held);
    let down = matches!(self.down, State::Pressed | State::Held);
    let left = matches!(self.left, State::Pressed | State::Held);
    let right = matches!(self.right, State::Pressed | State::Held);

    self.direction = match (up, down, left, right) {
      (true, false, true, false) => Some(Direction::NorthWest),
      (true, false, false, true) => Some(Direction::NorthEast),
      (true, false, false, false) => Some(Direction::North),
      (false, true, true, false) => Some(Direction::SouthWest),
      (false, true, false, true) => Some(Direction::SouthEast),
      (false, true, false, false) => Some(Direction::South),
      (false, false, true, false) => Some(Direction::West),
      (false, false, false, true) => Some(Direction::East),
      _ => {
        self.recenter();
        None
      }
    };
  }

  pub fn recenter(&mut self) {
    self.x = 0;
    self.y = 0;
    self.up = State::default();
    self.down = State::default();
    self.left = State::default();
    self.right = State::default();
    self.direction = None;
    self.last_event = Instant::now();
  }

  pub fn set_up(&mut self, up: State) {
    self.up = up;
  }

  pub fn set_down(&mut self, down: State) {
    self.down = down;
  }

  pub fn set_left(&mut self, left: State) {
    self.left = left;
  }

  pub fn set_right(&mut self, right: State) {
    self.right = right;
  }

  pub fn direction(&self) -> Option<Direction> {
    self.direction
  }

  pub fn x(&self) -> i32 {
    self.x
  }

  pub fn y(&self) -> i32 {
    self.y
  }

  pub fn up(&self) -> State {
    self.up
  }

  pub fn down(&self) -> State {
    self.down
  }

  pub fn left(&self) -> State {
    self.left
  }

  pub fn right(&self) -> State {
    self.right
  }

  pub fn last_event(&self) -> Instant {
    self.last_event
  }
}
