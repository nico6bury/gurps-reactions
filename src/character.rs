use serde::{Deserialize, Serialize};


/// Tracks one reaction modifier that might be attached to a character.
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Hash,Default,Serialize,Deserialize)]
pub struct ReactionMod {
	pub name: String,
	pub modi: i32,
	pub enabled: bool,
}//end struct RactionMod

impl ReactionMod {
	/// Creates a new Reaction Modifier object.  
	/// Starts out enabled.
	pub fn new(name: &str, modifier: i32) -> ReactionMod {
		ReactionMod {
			name: name.to_string(),
			modi: modifier,
			enabled: true,
		}//end struct construction
	}//end new()
}//end impl ReactionMod

/// A single character, which has a number of reaction modifiers.
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Hash,Default,Serialize,Deserialize)]
pub struct Character {
	pub name: String,
	pub reaction_modifiers: Vec<ReactionMod>,
}//end struct Character

impl Character {
	/// Creates a new character with no reaction modifiers.
	pub fn new(name: &str) -> Character {
		Character {
			name: name.to_string(),
			reaction_modifiers: Vec::new(),
		}//end struct construction
	}//end new()

	/// Sums up all the reaction modifiers on a character.  
	/// If count_disabled is false, then only reaction modifiers
	/// with enabled == true will be considered.  
	/// Otherwise, all modifiers will be considered regardless.
	/// 
	/// # Examples
	/// 
	/// ```
	/// # use gurps_reactions::character::ReactionMod;
	/// # use gurps_reactions::character::Character;
	/// let mut bob = Character::new("Bob");
	/// bob.reaction_modifiers.push(ReactionMod::new("Ugly", -6));
	/// bob.reaction_modifiers.push(ReactionMod::new("Kind", 1));
	/// assert_eq!(-5, bob.reaction_sum(true));
	/// assert_eq!(-5, bob.reaction_sum(false));
	/// ```  
	/// 
	/// ```
	/// # use gurps_reactions::character::ReactionMod;
	/// # use gurps_reactions::character::Character;
	/// let mut witch = Character::new("wicked witch");
	/// witch.reaction_modifiers.push(ReactionMod::new("Wicked",-2));
	/// let social_stigma = ReactionMod {name: "Stigma".to_string(), modi: -1, enabled: false};
	/// witch.reaction_modifiers.push(social_stigma);
	/// assert_eq!(-2, witch.reaction_sum(false));
	/// assert_eq!(-3, witch.reaction_sum(true));
	/// ```
	/// 
	pub fn reaction_sum(&self, count_disabled: bool) -> i32 {
		self.reaction_modifiers.iter()
			.fold(0, |acum, modi| {
				if count_disabled || modi.enabled {acum + modi.modi}
				else {acum}
			})
	}//end reaction_sum()
}//end impl for Character