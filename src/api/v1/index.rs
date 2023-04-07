use crate::api::v1;
use poem::{get, handler, post, Route};

pub fn endpoint() -> poem::Route {
  Route::new()
    .nest("/", package_version)
    .at("/users/:username", get(v1::user::get))
    .at("/user/create", post(v1::user::create))
    .at("/user/login", post(v1::user::login))
    .at("/session/:session", get(v1::auth::check))
}

#[handler]
pub fn package_version() -> String {
  format!("{{\"version\": \"{}\"}}", env!("CARGO_PKG_VERSION"))
}
