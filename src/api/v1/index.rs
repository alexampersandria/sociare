use crate::api::v1;
use poem::{get, handler, Route};

pub fn endpoint() -> poem::Route {
  Route::new()
    .nest("/", package_version)
    .at("/user/:username", get(v1::user::get))
}

#[handler]
pub fn package_version() -> String {
  format!("{{\"version\": \"{}\"}}", env!("CARGO_PKG_VERSION"))
}
