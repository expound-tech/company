#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::CompanyWebsite;

/// Time of day as seconds since midnight. Used for clock in app.
pub(crate) fn seconds_since_midnight() -> f64 {
    use chrono::Timelike as _;
    let time = chrono::Local::now().time();
    time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64)
}

pub trait PageApp {
    fn page_ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame);

    #[cfg(feature = "glow")]
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}
}