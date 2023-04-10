use crate::api::v1;
use poem::{delete, get, handler, patch, post, Route};

pub fn endpoint() -> poem::Route {
  Route::new()
    .nest("/", package_version)
    .at("/users/:username", get(v1::user::get))
    .at("/user/create", post(v1::user::create))
    .at("/user/delete", delete(v1::user::delete))
    .at("/user/edit", patch(v1::user::edit))
    .at("/user/login", post(v1::user::login))
    .at("/me", get(v1::user::me))
    .at("/sessions/:session", get(v1::auth::check))
    .at("/sessions", get(v1::auth::list))
    .at("/session/delete/:session", delete(v1::auth::delete))
    .at("/groups/:group", get(v1::group::get))
    .at("/groups/:group/edit", patch(v1::group::edit))
    .at("/groups", get(v1::group::get_all))
    .at("/group/create", post(v1::group::create))
    .at("/groups/:group/add", post(v1::group::add))
    .at("/groups/:group/remove", post(v1::group::remove))
}

#[handler]
pub fn package_version() -> String {
  format!("{{\"version\": \"{}\"}}", env!("CARGO_PKG_VERSION"))
}
