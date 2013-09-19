
// Copyright 2013 The GLFW-RS Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This example shows how shared objects can be accessed from callbacks
//!
//! This example should be seen as two different projects:
//!     * the first part is written as if we are the user of the middleware library.
//!     * the second part is written as if we are a middleware library writer. Typically, such
//!     middleware library can be a graphics engine.
//!
//! The goal is to show how the middleware library writer can write its event handling in a
//! flexible way for the user. That is, he must both use glfw callbacks to update the scene (here
//! the `State` object), and to call some custom event handling provided by the user.
//!
//! The user will try to do a single, simple thing: count the total number of cursor and mouse
//! events and display it on its main loop.

extern mod glfw;

use std::local_data;

/*
 * User app.
 */
#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

fn main() {
    // start the engine, open a window
    do Engine::spawn |engine| {
        // we need a counter. It must be @mut so that we can use it in every callback.
        let counter = @mut 0u;

        do State::set_key_event_callback {
            *counter = *counter + 1;
        }

        do State::set_mouse_event_callback {
            *counter = *counter + 1;
        }

        // start the main loop
        do engine.render_loop |_| {
            println(counter.to_str());
        }
    }
}

/*
 * Engine
 */
static tls_key: local_data::Key<@mut State> = &local_data::Key;

pub struct Engine {
    window: glfw::Window
}

impl Engine {
    pub fn spawn(callback: ~fn(&mut Engine)) {
        do glfw::set_error_callback |_, msg| {
            println!("GLFW Error: {:s}", msg);
        }

        do glfw::start {
            let window = glfw::Window::create(300, 300, "Move cursor in window", glfw::Windowed).unwrap();
            let mut engine = Engine {window: window };
            State::init(0f, 0f);

            do engine.window.set_cursor_pos_callback |_, x, y| {
                State::handle_mouse(x, y);
            }

            do engine.window.set_key_callback |win, key, _, action, _mods| {
                if action == glfw::PRESS && key == glfw::KEY_ESCAPE {
                    win.set_should_close(true);
                }
                else {
                    State::handle_keyboard();
                }
            }

            engine.window.make_context_current();

            callback(&mut engine);
        }
    }

    pub fn render_loop(&mut self, callback: &fn(&mut Engine)) {
        while !self.window.should_close() {
            glfw::poll_events();
            let (x, y) = State::get_pos();
            callback(self);
            println("Cursor pos: " + x.to_str() + ", " + y.to_str());
        }
    }
}

struct State {
    priv pos: (float, float),

    // Those two callbacks must be @ and must be in the ’State’ to make them callbable indirectly
    // by glfw callbacks.
    //
    // Note that those callbacks might be really useless/hard to use if they dont give the user
    // access to the Engine: @fn(&mut Engine).
    // But Engine, is not on TLS, so this is not possible! Anyway, assume the user does not need
    // the Engine to proceed (or has its own @mut Engine captured by its callback).
    priv usr_key_handler:   @fn(),
    priv usr_mouse_handler: @fn()
}

impl State {
    pub fn init(x: float, y: float) {
        local_data::set(
            tls_key,
            @mut State { pos: (x, y), usr_key_handler: || { }, usr_mouse_handler: || { } }
        );
    }

    pub fn handle_mouse(x: float, y: float) {
        do local_data::get(tls_key) |opt| {
            let state = opt.expect("Task-local state not initialized.");
            state.pos = (x, y);
            (state.usr_mouse_handler)();
        }
    }

    pub fn handle_keyboard() {
        do local_data::get(tls_key) |opt| {
            (opt.expect("Task-local state not initialized.").usr_key_handler)();
        }
    }

    pub fn get_pos() -> (float, float) {
        do local_data::get(tls_key) |opt| {
            opt.expect("Task-local state not initialized.").pos
        }
    }

    pub fn set_key_event_callback(callback: @fn()) {
        do local_data::get(tls_key) |opt| {
            opt.expect("Task-local state not initialized.").usr_key_handler = callback;
        }
    }

    pub fn set_mouse_event_callback(callback: @fn()) {
        do local_data::get(tls_key) |opt| {
            opt.expect("Task-local state not initialized.").usr_mouse_handler = callback;
        }
    }
}
