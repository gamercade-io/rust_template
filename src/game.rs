use gamercade_rs::{
    prelude::{self as gc, set_pixel},
    GraphicsParameters,
};

// Our game state. Edit this as you wish.
pub struct MyGame {
    frame_counter: usize,
    x_pos: i32,
    y_pos: i32,
}

impl crate::Game for MyGame {
    /// Handle all of your initialization logic here.
    fn init() -> Self {
        // We can call Gamercade functions in here.
        gc::console_log("Hello, world!");

        // Initialize our values to 0, and width/height divided
        // by two.
        Self {
            frame_counter: 0,
            x_pos: (gc::width() / 2) as i32,
            y_pos: (gc::height() / 2) as i32,
        }
    }

    /// Handle all of your game state logic here
    fn update(&mut self) {
        // Print a message if the user presses the A button.
        // This defaults to the U key on the keyboard.
        if Some(true) == gc::button_a_pressed(0) {
            gc::console_log("Pressed A.")
        }

        // Let's move the pixel with the arrow keys
        // Handle up/down motion
        if Some(true) == gc::button_up_held(0) {
            self.y_pos -= 1;
        } else if Some(true) == gc::button_down_held(0) {
            self.y_pos += 1;
        }

        // And repeat for left/right
        if Some(true) == gc::button_left_held(0) {
            self.x_pos -= 1;
        } else if Some(true) == gc::button_right_held(0) {
            self.x_pos += 1;
        }

        // Update the frame counter to keep the animation looping
        self.frame_counter += 1;
    }

    /// Handle all of your rendering code here
    fn draw(&self) {
        // Clear screen function takes a GraphicsParameters as a parameter,
        // so let's make one.
        let clear_color = GraphicsParameters::default().color_index(0);

        // Now we can clear the screen.
        gc::clear_screen(clear_color);

        // Let's draw a pixel.
        let pixel_color = GraphicsParameters::default().color_index(32);
        set_pixel(pixel_color, self.x_pos, self.y_pos);

        // Let's draw a spinning pixel.
        let spinning_pixel_color = GraphicsParameters::default().color_index(9);

        // Make it spin around
        let x = (self.frame_counter as f32 * 0.1).sin() * 25.0;
        let y = (self.frame_counter as f32 * 0.1).cos() * 25.0;

        let x = x as i32 + self.x_pos;
        let y = y as i32 + self.y_pos;

        // Draw the spinning pixel
        set_pixel(spinning_pixel_color, x, y);
    }
}
