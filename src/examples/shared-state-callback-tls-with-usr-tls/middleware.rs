use glfw;
use std::local_data;

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

    // callbacks are owned this time
    priv usr_key_handler:   ~fn(),
    priv usr_mouse_handler: ~fn()
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

    pub fn set_key_event_callback(callback: ~fn()) {
        do local_data::get(tls_key) |opt| {
            // XXX: "Cannot move out of captured outer variable"
            // Using a ~fn user defined callback does not seem even possible in combination with
            // TLS…
            opt.expect("Task-local state not initialized.").usr_key_handler = callback;
        }
    }

    pub fn set_mouse_event_callback(callback: ~fn()) {
        do local_data::get(tls_key) |opt| {
            // XXX: "Cannot move out of captured outer variable"
            // Using a ~fn user defined callback does not seem even possible in combination with
            // TLS…
            opt.expect("Task-local state not initialized.").usr_mouse_handler = callback;
        }
    }
}
