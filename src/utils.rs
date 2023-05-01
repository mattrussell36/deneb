use web_sys::console;

pub fn console_log(s: String) {
    console::log_1(&s.into());
}