use gui::{InterfaceMessage, GUI};

mod gui;

fn main() {
    println!("Hello, world!");
    let gui = GUI::initialize();
    let recv = gui.get_receiver();

    while gui.wait() {
        match recv.recv() {
            Some(InterfaceMessage::NewCharacter) => println!("New Character"),
            Some(InterfaceMessage::SaveCharacterAs) => println!("Save Character As"),
            Some(InterfaceMessage::OpenCharacter) => println!("Open Character"),
            None => {},
        }//end matching messages received
    }//end looping while gui is up
    println!("World ending!");
}//end main function
