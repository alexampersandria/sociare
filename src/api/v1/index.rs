use crate::api::v1;
use poem::{delete, get, handler, post, Route};

pub fn endpoint() -> poem::Route {
  Route::new()
    .nest("/", package_version)
    .at("/users/:username", get(v1::user::get))
    .at("/user/create", post(v1::user::create))
    .at("/user/login", post(v1::user::login))
    .at("/me", get(v1::user::me))
    .at("/sessions/:session", get(v1::auth::check))
    .at("/sessions", get(v1::auth::list))
    .at("/session/delete/:session", delete(v1::auth::delete))
}

#[handler]
pub fn package_version() -> String {
  format!("{{\"version\": \"{}\"}}", env!("CARGO_PKG_VERSION"))
}
