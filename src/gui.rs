use fltk::{app::{self, App, Receiver, Sender}, enums::{Align, FrameType, Shortcut}, frame::Frame, group::{Group, Pack, Scroll, Tile}, menu::{MenuFlag, SysMenuBar}, prelude::{DisplayExt, GroupExt, MenuExt, WidgetBase, WidgetExt}, text::{TextBuffer, TextDisplay}, window::Window};

mod character_box;

/// The width in pixels for the main window
const WINDOW_WIDTH: i32 = 850;
/// The height in pixels for the main window
const WINDOW_HEIGHT: i32 = 435;

/// The height in pixels for the top menu bar
const TOP_MENU_HEIGHT: i32 = 35;
/// The width in pixels for the list of characters. Should be less than WINDOW_WIDTH.
const CHARACTER_LIST_WIDTH: i32 = 500;
/// The padding to apply to elements within the character list
const CHARACTER_LIST_PADDING: i32 = 10;
/// The FrameType to use for major groups in the main window gui
const MAIN_GROUP_FRAME: FrameType = FrameType::GtkThinUpBox;

/// The width and height in pixels of each frame that shows a die result
const DIE_FRM_SIZE: i32 = 50;
/// The amount of pixels in padding to apply to each frame that shows a die result
const DIE_FRM_PADDING: i32 = 30;
/// The FrameType to use for the frames that show die results
const DIE_FRM_FRAME: FrameType = FrameType::GtkThinUpBox;
/// The alignment to use for the label on each frame that shows a die result
const DIE_FRM_ALIGN: Align = Align::Top;

/// The width and height in pixels of each frame that shows a reaction result
const RCT_FRM_SIZE: i32 = 65;
/// The amount of pixels in padding to apply to each frame that shows a reaction result
const RCT_FRM_PADDING: i32 = 30;
/// The FrameType to use for the frames that show reaction results
const RCT_FRM_FRAME: FrameType = FrameType::GtkThinUpBox;
/// The alignment to use for the label on each frame that shows a die result
const RCT_FRM_ALIGN: Align = Align::Inside;
/// The font size of the label for each frame that shows a reaction result
const RCT_FRM_LABEL_SIZE: i32 = 22;
/// The number of pixels in padding to apply to the textbox showing detailed reaction roll results.
const RCT_RST_TXT_PADDING: i32 = 20;

/// Holds all the stuff necessary for showing and interacting with the GUI.
#[allow(dead_code)]
pub struct GUI {
	app: App,
	ux_main_window: Window,
	msg_sender: Sender<InterfaceMessage>,
	msg_receiver: Receiver<InterfaceMessage>,
	ux_die_frm_1: Frame,
	ux_die_frm_2: Frame,
	ux_die_frm_3: Frame,
	ux_rct_frm_roll: Frame,
	ux_rct_frm_mod: Frame,
	ux_rct_frm_sum: Frame,
	ux_rct_frm_res: Frame,
	ux_rct_frm_result_txt_box: TextDisplay,
}//end struct GUI

impl GUI {
	/// Gives access to app.wait().  
	/// It is recommended to set up the main app loop
	/// using something like `while gui.wait() {}`
	pub fn wait(&self) -> bool {
		self.app.wait()
	}//end wait()

	/// Returns a reference to a receiver, used for the main function to
	/// get messages from the GUI/User.
	pub fn get_receiver(&self) -> &Receiver<InterfaceMessage> { &self.msg_receiver }

	/// Something like the init-components of other systems.  
	/// Sets up all the widgets and stuff for the GUI.
	pub fn initialize() -> GUI {
		let reaction_app = App::default();
		let mut main_window = Window::default()
			.with_size(WINDOW_WIDTH,WINDOW_HEIGHT)
			.with_label("GURPS Reaction Rolls Helper");
		main_window.end();
		main_window.make_resizable(true);

		let (s,r) = app::channel();
		
		let mut tiles = Tile::default()
			.with_pos(0,TOP_MENU_HEIGHT)
			.with_size(WINDOW_WIDTH, WINDOW_HEIGHT - TOP_MENU_HEIGHT);
		tiles.set_frame(MAIN_GROUP_FRAME);
		main_window.add_resizable(&tiles);

		let mut top_menu = SysMenuBar::default()
			.with_size(WINDOW_WIDTH, TOP_MENU_HEIGHT);
		top_menu.set_frame(FrameType::FlatBox);
		main_window.add(&top_menu);

		top_menu.add_emit(
			"Characters/New\t",
			Shortcut::None,
			MenuFlag::Normal,
			s,
			InterfaceMessage::NewCharacter
		);
		top_menu.add_emit(
			"Characters/Save As...\t",
			Shortcut::None,
			MenuFlag::Normal,
			s,
			InterfaceMessage::SaveCharacterAs
		);
		top_menu.add_emit(
			"Characters/Open...\t",
			Shortcut::None,
			MenuFlag::Normal,
			s,
			InterfaceMessage::OpenCharacter
		);

		// group for listing reaction rolls
		let mut reaction_roll_group = Group::default()
			.with_pos(CHARACTER_LIST_WIDTH, TOP_MENU_HEIGHT)
			.with_size(WINDOW_WIDTH - CHARACTER_LIST_WIDTH, tiles.height());
		reaction_roll_group.set_frame(MAIN_GROUP_FRAME);
		tiles.add(&reaction_roll_group);

		let mut die_frm_2 = Frame::default()
			.with_pos(reaction_roll_group.x() + (reaction_roll_group.width() / 2) - (DIE_FRM_SIZE / 2),TOP_MENU_HEIGHT + DIE_FRM_PADDING)
			.with_size(DIE_FRM_SIZE,DIE_FRM_SIZE)
			.with_label("die 2")
			.with_align(DIE_FRM_ALIGN);
		die_frm_2.set_frame(DIE_FRM_FRAME);
		reaction_roll_group.add(&die_frm_2);

		let mut die_frm_1 = Frame::default()
			.with_pos(reaction_roll_group.x() + ((die_frm_2.x() - reaction_roll_group.x()) / 2) - (DIE_FRM_SIZE / 2), TOP_MENU_HEIGHT + DIE_FRM_PADDING)
			.with_size(DIE_FRM_SIZE,DIE_FRM_SIZE)
			.with_label("die 1")
			.with_align(DIE_FRM_ALIGN);
		die_frm_1.set_frame(DIE_FRM_FRAME);
		reaction_roll_group.add(&die_frm_1);

		let mut die_frm_3 = Frame::default()
			.with_pos(die_frm_2.x() + die_frm_2.width() + ( (reaction_roll_group.x() + reaction_roll_group.width() - die_frm_2.x() - die_frm_2.width()) / 2) - (DIE_FRM_SIZE / 2), TOP_MENU_HEIGHT + DIE_FRM_PADDING)
			.with_size(DIE_FRM_SIZE,DIE_FRM_SIZE)
			.with_label("die 3")
			.with_align(DIE_FRM_ALIGN);
		die_frm_3.set_frame(DIE_FRM_FRAME);
		reaction_roll_group.add(&die_frm_3);

		let mut rct_frm_mod = Frame::default()
			.with_pos(reaction_roll_group.x() + (reaction_roll_group.width() / 2) - (RCT_FRM_SIZE / 2), die_frm_2.y() + die_frm_2.height() + DIE_FRM_PADDING.max(RCT_FRM_PADDING))
			.with_size(RCT_FRM_SIZE,RCT_FRM_SIZE)
			.with_label("8")
			.with_align(RCT_FRM_ALIGN);
		rct_frm_mod.set_frame(RCT_FRM_FRAME);
		rct_frm_mod.set_label_size(RCT_FRM_LABEL_SIZE);
		reaction_roll_group.add(&rct_frm_mod);

		let mut rct_frm_roll = Frame::default()
			.with_pos(reaction_roll_group.x() + ((rct_frm_mod.x() - reaction_roll_group.x()) / 2) - (RCT_FRM_SIZE / 2), rct_frm_mod.y())
			.with_size(RCT_FRM_SIZE,RCT_FRM_SIZE)
			.with_label("6")
			.with_align(RCT_FRM_ALIGN);
		rct_frm_roll.set_frame(RCT_FRM_FRAME);
		rct_frm_roll.set_label_size(RCT_FRM_LABEL_SIZE);
		reaction_roll_group.add(&rct_frm_roll);

		let mut rct_frm_sum = Frame::default()
			.with_pos(rct_frm_mod.x() + rct_frm_mod.width() + ( (reaction_roll_group.x() + reaction_roll_group.width() - rct_frm_mod.x() - rct_frm_mod.width()) / 2) - (RCT_FRM_SIZE / 2), rct_frm_mod.y())
			.with_size(RCT_FRM_SIZE,RCT_FRM_SIZE)
			.with_label("14")
			.with_align(RCT_FRM_ALIGN);
		rct_frm_sum.set_frame(RCT_FRM_FRAME);
		rct_frm_sum.set_label_size(RCT_FRM_LABEL_SIZE);
		reaction_roll_group.add(&rct_frm_sum);
		
		let mut rct_plus = Frame::default()
			.with_pos(rct_frm_roll.x() + rct_frm_roll.width(), rct_frm_roll.y())
			.with_size(rct_frm_mod.x() - rct_frm_roll.x() - rct_frm_roll.width(), rct_frm_roll.height())
			.with_label("@+")
			.with_align(RCT_FRM_ALIGN);
		rct_plus.set_frame(FrameType::FlatBox);
		reaction_roll_group.add(&rct_plus);

		let mut rct_eql = Frame::default()
			.with_pos(rct_frm_mod.x() + rct_frm_mod.width(), rct_frm_mod.y())
			.with_size(rct_frm_sum.x() - rct_frm_mod.x() - rct_frm_mod.width(), rct_frm_mod.height())
			.with_label("@2||")
			.with_align(RCT_FRM_ALIGN);
		rct_eql.set_frame(FrameType::FlatBox);
		reaction_roll_group.add(&rct_eql);

		let mut rct_nxt = Frame::default()
			.with_pos(rct_frm_mod.x(), rct_frm_mod.y() + rct_frm_mod.height())
			.with_size(rct_frm_mod.width(), rct_frm_mod.height() * 2 / 3)
			.with_label("@+52->")
			.with_align(RCT_FRM_ALIGN);
		rct_nxt.set_frame(FrameType::FlatBox);
		reaction_roll_group.add(&rct_nxt);

		let mut rct_frm_result = Frame::default()
			.with_pos(reaction_roll_group.x() + (reaction_roll_group.width() / 2) - (RCT_FRM_SIZE), rct_nxt.y() + rct_nxt.height())
			.with_size(RCT_FRM_SIZE * 2, RCT_FRM_SIZE / 2)
			.with_label("Good")
			.with_align(RCT_FRM_ALIGN);
		rct_frm_result.set_frame(RCT_FRM_FRAME);
		reaction_roll_group.add(&rct_frm_result);

		let mut rct_result_txt_buf = TextBuffer::default();
		let mut rct_result_txt_box = TextDisplay::default()
			.with_pos(reaction_roll_group.x() + (RCT_FRM_PADDING / 2), rct_frm_result.y() + rct_frm_result.height() + (RCT_RST_TXT_PADDING / 2))
			.with_size(reaction_roll_group.width() - RCT_FRM_PADDING, reaction_roll_group.height() - rct_nxt.y() - rct_nxt.height() - RCT_RST_TXT_PADDING);
		rct_result_txt_buf.set_text("General Reaction: ...\nPotential Combat: ...\nCommercial Transactions: ...\nRequests for Aid: ...\nRequests for Info: ...\nLoyalty: ...");
		rct_result_txt_box.set_buffer(rct_result_txt_buf);
		reaction_roll_group.add_resizable(&rct_result_txt_box);

		// group for listing characters
		let mut characters_scroll = Scroll::default()
			.with_pos(0,TOP_MENU_HEIGHT)
			.with_size(CHARACTER_LIST_WIDTH, tiles.height());
		characters_scroll.set_frame(MAIN_GROUP_FRAME);
		tiles.add_resizable(&characters_scroll);

		let mut character_pack = Pack::default()
			.with_pos(characters_scroll.x() + CHARACTER_LIST_PADDING, characters_scroll.y() + CHARACTER_LIST_PADDING)
			.with_size(characters_scroll.width() - (CHARACTER_LIST_PADDING * 2), 0);
		character_pack.set_spacing(CHARACTER_LIST_PADDING);
		character_pack.set_frame(FrameType::EmbossedFrame);
		characters_scroll.add_resizable(&character_pack);
		character_pack.resize_callback({
			move |pack,_,_,_,_| {
				match pack.parent() {
					None => {},
					Some(parent) => {
						let px = parent.x();
						let py = parent.y();
						let pw = parent.w();
						let ph = parent.h();
						let ex = px + CHARACTER_LIST_PADDING;
						let ey = py + CHARACTER_LIST_PADDING;
						let ew = pw - (2 * CHARACTER_LIST_PADDING);
						let eh = ph - (2 * CHARACTER_LIST_PADDING);
						if pack.x() != ex || pack.y() != ey {pack.set_pos(ex,ey);}
						if pack.w() != ew || pack.h() != eh {pack.set_size(ew,eh);}
					},
				}//end matching whether we can access the parent
			}//end closure
		});

		let mut ch1 = Frame::default()
			.with_size(0,50)
			.with_label("Character 1");
		ch1.set_frame(FrameType::GtkRoundUpFrame);
		character_pack.add(&ch1);

		let mut ch2 = Frame::default()
			.with_size(0,50)
			.with_label("Character 2");
		ch2.set_frame(FrameType::GtkRoundUpFrame);
		character_pack.add(&ch2);

		// TODO: Just add raw boxes of stuff for each character
		// TODO: Also add a box at the top with summed modifiers for the whole party, maybe with checkboxes and sums for each character
		// TODO: Also Also think about how to store the list of all current characters. Probably need to keep list of current characters and checks somehow

		main_window.show();
		GUI {
			app: reaction_app,
			ux_main_window: main_window,
			msg_sender: s,
			msg_receiver: r,
			ux_die_frm_1: die_frm_1,
			ux_die_frm_2: die_frm_2,
			ux_die_frm_3: die_frm_3,
			ux_rct_frm_roll: rct_frm_roll,
			ux_rct_frm_mod: rct_frm_mod,
			ux_rct_frm_sum: rct_frm_sum,
			ux_rct_frm_res: rct_frm_result,
			ux_rct_frm_result_txt_box: rct_result_txt_box,
		}//end struct construction
	}//end initialize()
}//end impl for GUI

/// Represents any messages that might be sent from GUI to
/// the main function.
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]
pub enum InterfaceMessage {
	/// Indicates that the user wants to create a new character
	NewCharacter,
	/// Indicates that the user wants to save a character file
	SaveCharacterAs,
	/// Indicates that the user wants to open a character file
	OpenCharacter,
}//end enum InterfaceMessage