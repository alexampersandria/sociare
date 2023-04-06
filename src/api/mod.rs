use crate::util;
use poem::{get, handler, Route};

#[handler]
fn api() -> String {
  format!("current_time: {}", util::unix_ms())
}

pub fn v1() -> poem::Route {
  Route::new().nest("/", get(api))
}
