//#![windows_subsystem = "windows"]
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

mod app;
pub mod csv_parse;

use app::BasicApp;
use nwg::NativeUi;

fn main() {
    nwg::init().expect("Could not initialize NWG");
    nwg::Font::set_global_family("Segoe UI").expect("Could not set default font");

    let _app = BasicApp::build_ui(Default::default()).expect("Could not build UI");
    nwg::dispatch_thread_events();
}
