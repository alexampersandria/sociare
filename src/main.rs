use poem::{
  endpoint::StaticFilesEndpoint,
  get, handler,
  listener::TcpListener,
  middleware::{NormalizePath, TrailingSlash},
  EndpointExt, Route, Server,
};
use sociare::util;

#[handler]
fn api() -> String {
  format!("current_time: {}", util::unix_ms())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  if std::env::var_os("RUST_LOG").is_none() {
    std::env::set_var("RUST_LOG", "poem=debug");
  }
  tracing_subscriber::fmt::init();

  let mut app = Route::new();

  app = app.at("/api/v1", get(api));

  app = app.at(
    "/",
    StaticFilesEndpoint::new("./www").index_file("index.html"),
  );

  Server::new(TcpListener::bind("127.0.0.1:3030"))
    .run(app.with(NormalizePath::new(TrailingSlash::Trim)))
    .await
}
