pub fn setup_logs() {
  std::env::set_var("RUST_LOG", "actix_web=warn");
  fast_log::init(fast_log::config::Config::new().console()).unwrap();
}
