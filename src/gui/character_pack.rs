use std::{cell::RefCell, rc::Rc};
use fltk::{button::CheckButton, enums::{Align, Event, FrameType}, frame::Frame, group::{Flex, FlexType, Pack}, input::Input, prelude::{GroupExt, InputExt, ValuatorExt, WidgetBase, WidgetExt}, valuator::{Counter, CounterType}, widget_extends};
use gurps_reactions::character::{Character, ReactionMod};

/// A gui widget that displays and allows editing of a single character
pub struct CharacterPack {
	pack: Pack,
	// pub ux_title_ref: Rc<RefCell<Frame>>,
	pub ux_char_name_box: Input,
	pub ux_mod_refs: Vec<ModifierLine>,
}//end struct CharacterPack

impl CharacterPack {
	/// Gets the character represented by this CharacterPack
	pub fn get_character(&self) -> Character {
		let mut modifiers = Vec::new();
		for mod_ref in self.ux_mod_refs.iter() {
			modifiers.push(mod_ref.get_mod_full());
		}//end getting each modifier for the character
		Character {
			name: self.ux_char_name_box.value(),
			reaction_modifiers: modifiers,
		}//end struct construction
	}//end get_character()

	/// Creates a new character pack out of the provided Character
	pub fn new(character: &Character) -> CharacterPack {
		let mut pack = Pack::default();
		pack.set_spacing(2);

		// add the title that auto-updates
		let mut char_title = Frame::default()
			.with_size(0, 30)
			.with_label(&format!("{}\t{}", character.name, character.reaction_sum(false)))
			.with_align(Align::Inside.union(Align::Center));
		char_title.set_frame(FrameType::FlatBox);
		pack.add(&char_title);

		// add the part with the name
		let mut char_name_flex = Flex::default()
			.with_size(0,30)
			.with_type(FlexType::Row);
		pack.add(&char_name_flex);
		let char_name_lbl = Frame::default()
			.with_label("Name:")
			.with_align(Align::Right.union(Align::Inside));
		char_name_flex.add(&char_name_lbl);
		char_name_flex.fixed(&char_name_lbl, 50);
		let mut char_name_box = Input::default()
			.with_size(0,30)
			.with_label("Name");
		char_name_box.set_align(Align::Inside.union(Align::Top));
		char_name_box.set_value(&character.name);
		char_name_box.set_tab_nav(true);
		char_name_flex.add(&char_name_box);

		// handlers and references for auto-updating title
		let char_title_ref = Rc::from(RefCell::from(char_title));
		char_name_box.handle({
			let char_title_ref = (&char_title_ref).clone();
			move |txt, ev| {
				match ev {
					Event::KeyDown => {
						let mut char_title = char_title_ref.borrow_mut();
						// let buf = txt.buffer().unwrap_or_else(|| {eprintln!("LOST BUFFER FROM TITLE!"); TextBuffer::default()});
						// separate character name from modifiers
						let mut split_label: Vec<String> = char_title.label().split("\t").map(|s| s.to_string()).collect();
						// make sure we have two components
						while split_label.len() < 2 {split_label.push("".to_string());}
						// draw the rest of the owl
						split_label[0] = txt.value();
						// split_label[0] = buf.text();
						// put everything back
						char_title.set_label(&split_label.join("\t"));

						true
					},
					_ => false
				}//end matching the event we catch
			}//end closure
		});

		// add the part with the checkboxes for each modifier
		let mut mod_lines = Vec::new();
		for modifier in character.reaction_modifiers.iter() {
			let mod_line = ModifierLine::new(modifier);
			pack.add(&*mod_line);
			// let mod_line_ref = Rc::from(RefCell::from(mod_box));
			mod_lines.push(mod_line);
		}//end creating gui widgets for each modifier

		CharacterPack {
			pack,
			// ux_title_ref: char_title_ref,
			ux_char_name_box: char_name_box,
			ux_mod_refs: mod_lines,
		}//end struct construction
	}//end new()
}//end impl for CharacterPack

widget_extends!(CharacterPack,Pack,pack);

pub struct ModifierLine {
	flex: Flex,
	pub mod_value: Counter,
	pub mod_check: CheckButton,
	pub mod_text: Input,
}//end struct ModifierLine

impl ModifierLine {
	/// Gets the modifier value for the represented ReactionMod.
	pub fn get_mod_value(&self) -> i32 { self.mod_value.value().ceil() as i32 }//end get_mod_value()

	/// Gets the modifier enabled-ness for the represented ReactionMod.
	pub fn get_mod_check(&self) -> bool { self.mod_check.is_checked() }//end get_mod_check()

	/// Gets the modifier text for the represented ReactionMod.
	pub fn get_mod_text(&self) -> String { self.mod_text.value() }//end get_mod_text()

	/// Gets the full ReactionMod represented.
	pub fn get_mod_full(&self) -> ReactionMod {
		ReactionMod {
			name: self.get_mod_text(),
			modi: self.get_mod_value(),
			enabled: self.get_mod_check(),
		}//end struct construction
	}//end get_mod_full()

	/// Creates a new ModifierLine which corresponds to a ReactionMod.
	pub fn new(modifier: &ReactionMod) -> ModifierLine {
		let mut mod_box = Flex::default()
			.with_size(0,25);
		mod_box.set_margin(0);

		let mut mod_value = Counter::default()
		.with_type(CounterType::Simple);
		mod_value.set_bounds(-99., 99.);
		mod_value.set_step(1., 1);
		mod_value.set_value(modifier.modi.into());
		mod_value.clear_visible_focus();
		mod_box.add(&mod_value);
		
		let mut mod_check = CheckButton::default();
		mod_check.clear_visible_focus();
		mod_check.set_checked(modifier.enabled);
		mod_box.add(&mod_check);
		mod_box.fixed(&mod_check, 30);
		mod_box.fixed(&mod_value, 50);
		
		let mut mod_text = Input::default();
		mod_text.set_value(&modifier.name);

		mod_box.end();

		ModifierLine {
			flex: mod_box,
			mod_value,
			mod_check,
			mod_text,
		}
	}//end new()
}//end impl for ModifierLine

widget_extends!(ModifierLine,Flex,flex);
