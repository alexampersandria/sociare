use crate::api::v1;
use poem::{delete, get, handler, patch, post, Route};

// allow longer lines for the sake of readability
#[rustfmt::skip]
pub fn endpoint() -> poem::Route {
  Route::new()
    .at("/", package_version)
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
    .at("/messages/:message", get(v1::message::get))
    .at("/message/create", post(v1::message::create))
    .at("/messages/:message/edit", patch(v1::message::edit))
    .at("/receipts/:receipt", get(v1::receipt::get))
    .at("/receipts/:receipt/edit", patch(v1::receipt::edit))
    .at("/receipt/create", post(v1::receipt::create))
    .at("/transactions/:transaction", get(v1::transaction::get))
    .at("/transaction/create", post(v1::transaction::create))
    .at("/transactions/:transaction/edit", patch(v1::transaction::edit))
    .at("/transactions/:transaction/confirm", patch(v1::transaction::confirm))
}

#[handler]
pub fn package_version() -> String {
  format!("{{\"version\": \"{}\"}}", env!("CARGO_PKG_VERSION"))
}
