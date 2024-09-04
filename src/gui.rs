use fltk::{app::App, prelude::WidgetExt, window::Window};

/// Holds all the stuff necessary for showing and interacting with the GUI.
#[allow(dead_code)]
pub struct GUI {
	app: App,
	ux_main_window: Window
}//end struct GUI

impl GUI {
	/// Gives access to app.wait().  
	/// It is recommended to set up the main app loop
	/// using something like `while gui.wait() {}`
	pub fn wait(&self) -> bool {
		self.app.wait()
	}//end wait()

	/// Something like the init-components of other systems.  
	/// Sets up all the widgets and stuff for the GUI.
	pub fn initialize() -> GUI {
		let reaction_app = App::default();
		let mut main_window = Window::default()
			.with_size(700,435)
			.with_label("GURPS Reaction Rolls Helper");

		main_window.show();
		GUI {
			app: reaction_app,
			ux_main_window: main_window,
		}//end struct construction
	}//end initialize()
}//end impl for GUI