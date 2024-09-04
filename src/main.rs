use gui::GUI;

mod gui;

fn main() {
    println!("Hello, world!");
    let gui = GUI::initialize();

    while gui.wait() {

    }//end looping while gui is up
    println!("World ending!");
}//end main function
