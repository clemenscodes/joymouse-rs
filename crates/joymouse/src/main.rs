fn main() {
  #[cfg(not(windows))]
  {
    platform::linux::Controller::run()
  }
  #[cfg(windows)]
  {
    println!("Windows is not supported yet. Stay tuned :)");
  }
}
