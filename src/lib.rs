use gamercade_rs::{prelude as gc, GraphicsParameters};

#[no_mangle]
/// Handle all of your initialization logic here.
pub extern "C" fn init() {
    gc::console_log("Hello, world!")
}

#[no_mangle]
/// Handle all of your game state logic here
pub extern "C" fn update() {
    // Print a message if the user presses the A button.
    // This defaults to the U key on the keyboard.
    if Some(true) == gc::button_a_pressed(0) {
        gc::console_log("Pressed A.")
    }
}

#[no_mangle]
/// Handle all of your rendering code here
pub extern "C" fn draw() {
    // Clear screen function takes a GraphicsParameters as a parameter,
    // so let's make one.
    let clear_color = GraphicsParameters::default().color_index(0);

    // Now we can clear the screen.
    gc::clear_screen(clear_color);
}
