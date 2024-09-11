use gui::{InterfaceMessage, GUI};
use gurps_reactions::character::{Character, ReactionMod};

mod gui;

fn main() {
    eprintln!("Hello, world!");
    let mut gui = GUI::initialize();
    let recv = gui.get_receiver();

    while gui.wait() {
        match recv.recv() {
            Some(InterfaceMessage::NewCharacter) => {
                let mut bob = Character::new("bob");
                bob.reaction_modifiers.push(ReactionMod::new("Ugly",-6));
                bob.reaction_modifiers.push(ReactionMod::new("Kind", 2));
                gui.set_character_display(&vec![bob]);
            },
            Some(InterfaceMessage::SaveCharacterAs) => println!("Save Character As"),
            Some(InterfaceMessage::OpenCharacter) => println!("Open Character"),
            Some(InterfaceMessage::EditCharacter) => {
                let characters = gui.get_characters();
                gui.set_character_display(&characters);
            }
            None => {},
        }//end matching messages received
    }//end looping while gui is up
    eprintln!("World ending!");
}//end main function
