fn main() {
  #[cfg(not(windows))]
  {
    platform::linux::Controller::run()
  }
  #[cfg(windows)]
  {
    platform::windows::Controller::run()
  }
}
