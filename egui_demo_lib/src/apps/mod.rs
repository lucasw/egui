pub mod color_test;
pub mod demo;
pub mod fractal_clock;
#[cfg(feature = "http")]
mod http_app;
pub mod node_graph;

// TODO: remove
pub use color_test::ColorTest;
pub use demo::DemoApp;
pub use fractal_clock::FractalClock;
#[cfg(feature = "http")]
pub use http_app::HttpApp;

pub use demo::DemoWindows; // used for tests
