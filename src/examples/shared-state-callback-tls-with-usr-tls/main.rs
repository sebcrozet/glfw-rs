
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
//!     * `middleware.rs` is written as if we are a middleware library writer. Typically, such
//!     middleware library can be a graphics engine.
//!     * `main.rs` (this file) is written as if we are the user of the middleware library.
//!
//! The goal is to show how the middleware library writer can write its event handling in a
//! flexible way for the user. That is, he must both use glfw callbacks to update the scene (here
//! the `State` object), and to call some custom event handling provided by the user.
//!
//! The user will try to do a single, simple thing: count the total number of cursor and mouse
//! events and display it on its main loop.

extern mod glfw;
use std::local_data;
use middleware::Engine;
use middleware::State;

mod middleware;

// since the library callbacks ar ~fn, we have to put the counter on TLS.
// The value could be an uint instead of @mut uint. But doing so makes it really painful to use.
static counter_tls_key: local_data::Key<@mut uint> = &local_data::Key;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

fn main() {
    // start the engine, open a window
    do Engine::spawn |engine| {
        // we need a counter. It must be @mut so that we can use it in every callback.
        local_data::set(counter_tls_key, @mut 0u);

        do State::set_key_event_callback {
            do local_data::get(counter_tls_key) |counter| {
                let c = *counter.unwrap();

                *c = *c + 1;
            }
        }

        do State::set_mouse_event_callback {
            do local_data::get(counter_tls_key) |counter| {
                let c = *counter.unwrap();

                *c = *c + 1;
            }
        }

        // start the main loop
        do engine.render_loop |_| {
            do local_data::get(counter_tls_key) |counter| {
                let c = counter.unwrap();
                println(c.to_str());
            }
        }
    }
}
