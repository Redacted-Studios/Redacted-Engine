// ---------------------------------------- Copyright 2022, Redacted Studios, All rights reserved ---------------------------------------- //
// Project:      Redacted Engine                                                                                                           //
// File:         main.rs                                                                                                                   //
// Developer(s): Redacted Studios, Nathan Russell                                                                                          //
// Purpose:      Manage the applications entry point                                                                                       //
// Notes:        None                                                                                                                      //
// --------------------------------------------------------------------------------------------------------------------------------------- //

mod engine;
// use glfw::{Action, Context, Key};

fn main() {
    let mut RWindowInstane = engine::window::RWindow { instance: None, window: None, events: None, width: 1280, height: 720, caption: "RWindow instance!".to_string() };
}

/*
fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
*/
