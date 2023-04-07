use crate::api::v1;
use poem::{get, handler, post, Route};

pub fn endpoint() -> poem::Route {
  Route::new()
    .nest("/", package_version)
    .at("/user/:username", get(v1::user::get))
    .at("/user", post(v1::user::post))
}

#[handler]
pub fn package_version() -> String {
  format!("{{\"version\": \"{}\"}}", env!("CARGO_PKG_VERSION"))
}
