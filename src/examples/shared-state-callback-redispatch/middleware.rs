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
