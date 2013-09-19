
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
//!     * the second part is written as if we are a middleware library writer.  Typically, such
//!     middleware library can be a graphics engine.
//!     * the first part is written as if we are the user of the middleware library.
//!
//! The goal is to show how the middleware library writer can write its event handling in a
//! flexible way for the user. That is, he must both use glfw callbacks to update the scene (here
//! the `State` object), and to call some custom event handling provided by the user.
//!
//! The user will try to do a single, simple thing: count the total number of cursor and mouse
//! events and display it on its main loop.

extern mod glfw;
extern mod extra;
use middleware::Engine;

mod middleware;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

fn main() {
    // start the engine, open a window
    do Engine::spawn |engine| {
        // we need a counter. It must be @mut so that we can use it in every callback.
        let mut counter = 0u;

        // start the main loop
        do engine.render_loop(|_, _| event_handler(&mut counter)) |_| {
            println(counter.to_str());
        }
    }
}

fn event_handler(counter: &mut uint) {
    *counter = *counter + 1;
}
use extra::arc::RWArc;
use glfw;

// enum used to wrap every glfw events.
enum Event {
    MouseEvent(float, float),
    KeyboardEvent(i32, i32)
}

pub struct Engine {
    window:          glfw::Window,
    state:           State,
    // we use this to collect every event at each frame
    // NOTE: we could put that on TLS instead of using a RWArc.
    event_collector: RWArc<~[Event]>
}

impl Engine {
    pub fn spawn(callback: ~fn(&mut Engine)) {
        do glfw::set_error_callback |_, msg| {
            println!("GLFW Error: {:s}", msg);
        }

        do glfw::start {
            let window = glfw::Window::create(300, 300, "Move cursor in window", glfw::Windowed).unwrap();
            let mut engine = Engine {
                window:          window,
                state:           State::new(0.0, 0.0),
                event_collector: RWArc::new(~[])
            };

            let ec = engine.event_collector.clone();
            do engine.window.set_cursor_pos_callback |_, x, y| {
                ec.write(|events| events.push(MouseEvent(x, y)));
            }

            let ec = engine.event_collector.clone();
            do engine.window.set_key_callback |_, key, _, action, _mods| {
                ec.write(|events| events.push(KeyboardEvent(key, action)));
            }

            engine.window.make_context_current();

            callback(&mut engine);
        }
    }

    pub fn render_loop(&mut self,
                       event_handler: &fn(&mut Engine, &Event),
                       callback:      &fn(&mut Engine)) {
        while !self.window.should_close() {
            // collect every event ...
            glfw::poll_events();
            // ... and redispatch them
            let collector = self.event_collector.clone();
            do collector.read |events| {
                for e in events.iter() {
                    // let the user handle the event ...
                    event_handler(self, e);

                    // .. and do our own handling too
                    match *e {
                        MouseEvent(x, y) => {
                            self.state.handle_mouse(x, y);
                        },
                        KeyboardEvent(key, action) => {
                            if action == glfw::PRESS && key == glfw::KEY_ESCAPE {
                                self.window.set_should_close(true);
                            }
                            else {
                                self.state.handle_keyboard();
                            }
                        }
                    }
                }
            }

            // clear the event collector
            self.event_collector.write(|events| events.clear());

            let (x, y) = self.state.get_pos();
            callback(self);
            println("Cursor pos: " + x.to_str() + ", " + y.to_str());
        }
    }
}

struct State {
    priv pos: (float, float),
}

impl State {
    pub fn new(x: float, y: float) -> State {
        State {
            pos: (x, y)
        }
    }

    pub fn handle_mouse(&mut self, x: float, y: float) {
        self.pos = (x, y);
    }

    pub fn handle_keyboard(&mut self) {
    }

    pub fn get_pos(&self) -> (float, float) {
        self.pos
    }
}
