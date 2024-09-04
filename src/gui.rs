use fltk::{app::App, enums::FrameType, group::Tile, menu::SysMenuBar, prelude::{GroupExt, MenuExt, WidgetExt}, window::Window};

/// The width in pixels for the main window
const WINDOW_WIDTH: i32 = 700;
/// The height in pixels for the main window
const WINDOW_HEIGHT: i32 = 435;
/// The height in pixels for the top menu bar
const TOP_MENU_HEIGHT: i32 = 35;
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
			.with_size(WINDOW_WIDTH,WINDOW_HEIGHT)
			.with_label("GURPS Reaction Rolls Helper");
		main_window.end();

		let top_menu = SysMenuBar::default()
			.with_size(WINDOW_WIDTH, TOP_MENU_HEIGHT);
		main_window.add(&top_menu);

		let tiles = Tile::default()
			.with_pos(0,TOP_MENU_HEIGHT)
			.with_size(WINDOW_WIDTH, WINDOW_HEIGHT - TOP_MENU_HEIGHT);
		main_window.add(&tiles);

		main_window.show();
		GUI {
			app: reaction_app,
			ux_main_window: main_window,
		}//end struct construction
	}//end initialize()
}//end impl for GUI

pub enum InterfaceMessage {

}