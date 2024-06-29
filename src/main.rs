use easy_dctl::MyEguiApp;

fn main() {
	let native_options = eframe::NativeOptions::default();	// sets default options on eframe
	let _ = eframe::run_native("easyDCTL", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));	// starts MyEguiApp from lib.rs
}
