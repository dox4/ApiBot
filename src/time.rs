use chrono::Local;

const _FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3fZ";

pub(crate) fn now() -> String {
    Local::now().format(_FORMAT).to_string()
}
