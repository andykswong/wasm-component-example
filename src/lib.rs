#[allow(warnings)]
mod bindings;

use bindings::host::log;
use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "guest: Hello, World!".to_string()
    }

    fn log_hello() {
        log("Hello, World!");
    }
}

bindings::export!(Component with_types_in bindings);
