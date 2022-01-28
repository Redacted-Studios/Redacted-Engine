// ---------------------------------------- Copyright 2022, Redacted Studios, All rights reserved ---------------------------------------- //
// Project:      Redacted Engine                                                                                                           //
// File:         engine/window/mod.rs                                                                                                      //
// Developer(s): Redacted Studios, Nathan Russell                                                                                          //
// Purpose:      Manage the Redacted Engines window(s)                                                                                     //
// Notes:        None                                                                                                                      //
// --------------------------------------------------------------------------------------------------------------------------------------- //

use glfw::{Action, Context, Key, WindowEvent};

// RWindow : Redacted Engine Window instance.
pub struct RWindow {
	pub instance: glfw::Glfw,
	pub window: glfw::Window,
	pub events: std::sync::mpsc::Receiver<(f64, WindowEvent)>,
	pub width: i32,
	pub height: i32,
	pub caption: String
}

impl RWindow {

	// init : Initialize the RWindow instance.
	pub fn init(&mut self) -> () {

		self.instance = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

		(self.window, self.events) = self.instance.create_window(1280, 720, "Redacted", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

	}

}
