use controller::PlatformControllerManager;

fn main() {
  #[cfg(not(windows))]
  {
    platform::linux::Controller::run().unwrap()
  }
  #[cfg(windows)]
  {
    platform::windows::Controller::run().unwrap()
  }
}
