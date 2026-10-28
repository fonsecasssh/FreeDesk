use hbb_common::config;

pub const APP_NAME: &str = "FreeDesk";

// `APP_NAME` lives in the hbb_common submodule, which this fork does not own,
// so it is overridden at startup through the same hook custom clients use.
pub fn apply() {
    *config::APP_NAME.write().unwrap() = APP_NAME.to_owned();
}
